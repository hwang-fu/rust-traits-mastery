#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

/// A user where equality is based ONLY on id, not name.
/// Two users are "equal" if they have the same id, even if names differ.
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        let name = String::from(name);
        let email = String::from(email);
        User { id, name, email }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq_derive() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(10, 20);
        let p3 = Point::new(10, 99);

        // == and != are enabled by PartialEq
        assert!(p1 == p2);
        assert!(p1 != p3);

        // assert_eq! uses PartialEq + Debug
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_partial_eq_manual() {
        let user1 = User::new(1, "Alice", "alice@example.com");
        let user2 = User::new(1, "Alice Updated", "newalice@example.com");
        let user3 = User::new(2, "Alice", "alice@example.com");

        // Same id = equal (even though name/email differ)
        assert_eq!(user1, user2);

        // Different id = not equal (even though name matches)
        assert_ne!(user1, user3);
    }
}
