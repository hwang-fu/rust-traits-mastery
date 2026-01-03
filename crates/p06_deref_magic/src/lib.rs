// ------------------------------

use std::ops::{Deref, DerefMut};

pub struct MyBox<T> {
    value: Box<T>,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let value = Box::new(value);
        MyBox { value }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

// ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_creation() {
        let boxed = MyBox::new(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_deref() {
        let boxed = MyBox::new(42);

        // Explicit dereference with *
        assert_eq!(*boxed, 42);

        fn takes_ref(n: &i32) -> i32 {
            *n * 2
        }

        assert_eq!(takes_ref(&boxed), 84);
    }

    #[test]
    fn test_deref_mut() {
        let mut boxed = MyBox::new(10);

        // Modify through mutable dereference
        *boxed = 20;

        assert_eq!(*boxed, 20);
    }

    #[test]
    fn test_deref_mut_coercion() {
        let mut boxed = MyBox::new(String::from("hello"));

        // &mut MyBox<String> coerces to &mut String
        fn modify_string(s: &mut String) {
            s.push_str(" world");
        }

        modify_string(&mut boxed);

        assert_eq!(&*boxed, "hello world");
    }
}

// ------------------------------
