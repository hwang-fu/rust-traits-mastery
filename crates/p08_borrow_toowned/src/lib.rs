use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    hash::Hash,
};

// ------------------------------

#[derive(Debug, Clone, Eq)]
pub struct CaseInsensitiveKey {
    value: String,
}

impl CaseInsensitiveKey {
    pub fn new(s: &str) -> Self {
        let value = s.into();
        CaseInsensitiveKey { value }
    }
}

impl Hash for CaseInsensitiveKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_lowercase().hash(state);
    }
}

impl PartialEq for CaseInsensitiveKey {
    fn eq(&self, other: &Self) -> bool {
        self.value.to_lowercase() == other.value.to_lowercase()
    }
}

impl PartialEq<str> for CaseInsensitiveKey {
    fn eq(&self, other: &str) -> bool {
        self.value.to_lowercase() == other.to_lowercase()
    }
}

impl Borrow<str> for CaseInsensitiveKey {
    fn borrow(&self) -> &str {
        &self.value
    }
}

pub fn lookup_demo() {
    let mut map = HashMap::<String, i32>::new();

    map.insert(String::from("apple"), 1);
    map.insert(String::from("banana"), 2);

    // Look up with &str - works because String: Borrow<str>
    let _ = map.get("apple"); // No allocation needed!
    let _ = map.get("banana");
}

// ------------------------------

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Buffer { data }
    }
}

impl Borrow<[u8]> for Buffer {
    fn borrow(&self) -> &[u8] {
        &self.data
    }
}

impl BorrowMut<[u8]> for Buffer {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

// ------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_case_insensitive_key_equality() {
        let key1 = CaseInsensitiveKey::new("Hello");
        let key2 = CaseInsensitiveKey::new("HELLO");
        let key3 = CaseInsensitiveKey::new("hello");
        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
    }

    #[test]
    fn test_hashmap_with_custom_key() {
        let mut map: HashMap<CaseInsensitiveKey, i32> = HashMap::new();

        map.insert(CaseInsensitiveKey::new("Hello"), 42);

        assert_eq!(map.get(&CaseInsensitiveKey::new("hellO")), Some(&42));
        assert_eq!(map.get(&CaseInsensitiveKey::new("HELLO")), Some(&42));
    }

    #[test]
    fn test_string_borrow_str() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert(String::from("key"), 100);

        // Look up with &str (no allocation!)
        assert_eq!(map.get("key"), Some(&100));

        // Also works with String
        assert_eq!(map.get(&String::from("key")), Some(&100));
    }

    #[test]
    fn test_buffer_borrow_mut() {
        let mut buf = Buffer::new(vec![1, 2, 3]);

        let slice: &[u8] = buf.borrow();
        assert_eq!(slice, &[1, 2, 3]);

        let slice_mut: &mut [u8] = buf.borrow_mut();
        slice_mut[0] = 99;

        assert_eq!(buf.borrow() as &[u8], &[99, 2, 3]);
    }
}

// ------------------------------
