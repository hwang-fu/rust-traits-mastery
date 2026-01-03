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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String); // e.g., "12345"

impl UserId {
    pub fn new(id: &str) -> Self {
        UserId(id.into())
    }

    pub fn display(&self) -> String {
        format!("USER-{}", self.0)
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// WRONG to implement Borrow<str>!
/*
    impl Borrow<str> for UserId {
        fn borrow(&self) -> &str {
            &self.0
        }
    }
*/
// Why wrong? Because:
//   UserId::new("123").hash() != "123".hash()
//
// UserId hashes the STRUCT (derived Hash), while str hashes just the string.
// HashMap lookups would FAIL silently!

// ------------------------------

/// A string wrapper where Borrow<str> IS correct.
/// Hash and Eq are based ONLY on the inner string.
#[derive(Debug, Clone)]
pub struct NormalizedString {
    value: String,
}

impl NormalizedString {
    pub fn new(s: &str) -> Self {
        NormalizedString {
            value: s.trim().to_lowercase(),
        }
    }
}

// Hash ONLY the inner string
impl Hash for NormalizedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// Eq ONLY compares inner string
impl PartialEq for NormalizedString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for NormalizedString {}

// Now Borrow<str> is CORRECT - Hash/Eq match!
impl Borrow<str> for NormalizedString {
    fn borrow(&self) -> &str {
        &self.value
    }
}

// ------------------------------
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

    #[test]
    fn test_asref_vs_borrow_userid() {
        // UserId uses AsRef<str>, NOT Borrow<str>
        let id = UserId::new("12345");

        // AsRef works for general string access
        fn takes_asref<S: AsRef<str>>(s: S) -> usize {
            s.as_ref().len()
        }
        assert_eq!(takes_asref(&id), 5);

        // But we CANNOT use UserId as HashMap key looked up by &str
        // because it doesn't (and shouldn't) implement Borrow<str>
        let mut map: HashMap<UserId, i32> = HashMap::new();
        map.insert(UserId::new("123"), 42);

        // Must look up with UserId, not &str
        assert_eq!(map.get(&UserId::new("123")), Some(&42));
        // map.get("123")  // Won't compile! UserId: !Borrow<str>
    }

    #[test]
    fn test_borrow_normalized_string() {
        // NormalizedString correctly implements Borrow<str>
        let mut map: HashMap<NormalizedString, i32> = HashMap::new();

        map.insert(NormalizedString::new("  Hello  "), 1);
        map.insert(NormalizedString::new("WORLD"), 2);

        // Can look up with &str because Hash/Eq are consistent!
        assert_eq!(map.get("hello"), Some(&1)); // Normalized: "hello"
        assert_eq!(map.get("world"), Some(&2)); // Normalized: "world"
    }
}

// ------------------------------
