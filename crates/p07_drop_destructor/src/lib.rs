// ------------------------------

use std::{cell::Cell, rc::Rc};

pub struct LoggingResource {
    name: String,
    drop_log: Option<Rc<Cell<Vec<String>>>>,
}

impl LoggingResource {
    pub fn new(name: &str) -> Self {
        println!("[CREATED] {}", name);
        let name = name.to_string();
        let drop_log = None;
        LoggingResource { name, drop_log }
    }

    pub fn with_log(name: &str, log: Rc<Cell<Vec<String>>>) -> Self {
        println!("[CREATED] {}", name);
        let name = name.to_string();
        let drop_log = Some(log);
        LoggingResource { name, drop_log }
    }
}

impl Drop for LoggingResource {
    fn drop(&mut self) {
        println!("[DROPPED] {}", self.name);

        if let Some(log) = self.drop_log.as_mut() {
            let mut entries = log.take();
            entries.push(self.name.clone());
            log.set(entries);
        }
    }
}

// ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_called_on_scope_exit() {
        let log = Rc::new(Cell::new(Vec::new()));

        {
            let _resource = LoggingResource::with_log("test", Rc::clone(&log));
            assert_eq!(log.take().len(), 0);
            log.set(Vec::new());
        } // dropped here

        let entries = log.take();
        assert_eq!(entries, vec!["test"]);
    }
}

// ------------------------------
