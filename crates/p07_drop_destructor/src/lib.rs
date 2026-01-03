// ------------------------------

use std::{cell::Cell, rc::Rc, time::Instant};

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

pub struct TimerGuard {
    name: String,
    start: Instant,
}

impl TimerGuard {
    pub fn new(name: &str) -> Self {
        println!("[TIMER START] {}", name);
        let name = name.to_string();
        let start = Instant::now();
        TimerGuard { name, start }
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}

impl Drop for TimerGuard {
    fn drop(&mut self) {
        println!(
            "[TIMER END] {} - took {:?}",
            self.name,
            self.start.elapsed()
        );
    }
}

// ------------------------------

// ------------------------------
// Note: Copy and Drop CANNOT coexist
// ------------------------------
//
// This would fail to compile:
//
// #[derive(Copy, Clone)]
// struct Broken {
//     data: i32,
// }
//
// impl Drop for Broken {
//     fn drop(&mut self) { }
// }
//
// ERROR: the trait `Copy` cannot be implemented for this type
//        because the type has a destructor
//
// Why? Copy means bitwise duplication. If Drop exists, which
// copy gets dropped? Both? That's a double-free. Rust prevents this.
// ------------------------------

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

    #[test]
    fn test_drop_order_reverse() {
        // Variables drop in REVERSE declaration order
        let log = Rc::new(Cell::new(Vec::new()));

        {
            let _first = LoggingResource::with_log("first", Rc::clone(&log));
            let _second = LoggingResource::with_log("second", Rc::clone(&log));
            let _third = LoggingResource::with_log("third", Rc::clone(&log));
            // Drop order: third, second, first (reverse!)
        }

        let entries = log.take();
        assert_eq!(entries, vec!["third", "second", "first"]);
    }

    #[test]
    fn test_early_drop_with_mem_drop() {
        let log = Rc::new(Cell::new(Vec::new()));

        let resource = LoggingResource::with_log("early", Rc::clone(&log));
        assert_eq!(Rc::strong_count(&log), 2);

        drop(resource); // Early drop

        assert_eq!(Rc::strong_count(&log), 1);

        let entries = log.take();
        assert_eq!(entries, vec!["early"]);
        log.set(Vec::new());
    }

    #[test]
    fn test_timer_guard() {
        {
            let timer = TimerGuard::new("test_operation");

            // Simulate some work
            std::thread::sleep(std::time::Duration::from_millis(10));

            // Timer still running
            assert!(timer.elapsed_ms() >= 10);
        }
        // Timer prints elapsed time here on drop
    }
}

// ------------------------------
