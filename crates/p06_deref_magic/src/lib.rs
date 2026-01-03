// ------------------------------

use std::{
    ops::{Deref, DerefMut},
    usize,
};

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

pub struct MyString {
    inner: String,
    modification_count: usize,
}

impl MyString {
    pub fn new(s: impl Into<String>) -> Self {
        let inner: String = s.into();
        let modification_count: usize = 0;
        MyString {
            inner,
            modification_count,
        }
    }

    pub fn modifications(&self) -> usize {
        self.modification_count
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
        self.modification_count += 1;
    }
}

impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.modification_count += 1;
        &mut self.inner
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

    #[test]
    fn test_mystring_deref_to_str() {
        let s = MyString::new("hello");

        assert_eq!(s.len(), 5); // str.len()
        assert!(s.contains("ello")); // str.contains()
        assert!(s.starts_with("he")); // str.starts_with()
        assert_eq!(&s[0..2], "he"); // str indexing
    }

    #[test]
    fn test_mystring_coercion() {
        let s = MyString::new("rust");

        fn take_str(s: &str) -> usize {
            s.len()
        }

        assert_eq!(take_str(&s), 4);
    }

    #[test]
    fn test_mystring_modification_tracking() {
        let mut s = MyString::new("hello");
        assert_eq!(s.modifications(), 0);

        s.push_str(" world");
        assert_eq!(s.modifications(), 1);
        assert_eq!(&*s, "hello world");
    }
}

// ------------------------------
