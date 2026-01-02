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

impl PartialEq<u64> for User {
    fn eq(&self, other: &u64) -> bool {
        self.id == *other
    }
}

/// A type with both PartialEq and Eq.
/// Eq guarantees: a == a is ALWAYS true (reflexive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductId(pub u64);

/// Demonstrating why f64 cannot be Eq.
/// NaN (Not a Number) breaks reflexivity: NaN != NaN
#[derive(Debug, Clone, PartialEq)] // No Eq! f64 inside
pub struct Measurement {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score(u32);

impl Score {
    pub fn new(value: u32) -> Self {
        Score(value)
    }
}

impl PartialOrd for Score {
    // it automatically implements .lt(), .le(), .gt(), .ge() (<, <=, >, >=) based on this method
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
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

    #[test]
    fn test_eq_marker() {
        let id1 = ProductId(100);
        let id2 = ProductId(100);

        // Eq types can be compared and are reflexive
        assert_eq!(id1, id1); // reflexive: a == a
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_nan_breaks_equality() {
        let m1 = Measurement { value: f64::NAN };

        // NaN is NOT equal to itself! This is why f64 can't be Eq.
        assert!(m1.value != m1.value); // NaN != NaN
        assert!(m1 != m1); // Therefore Measurement != itself
    }

    #[test]
    fn test_ord_derive() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 2, 0);
        let v3 = Version::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);

        // Sorting works automatically
        let mut versions = vec![v3.clone(), v1.clone(), v2.clone()];
        versions.sort();
        assert_eq!(versions, vec![v1, v2, v3]);
    }

    #[test]
    fn test_ord_manual_partialord() {
        let s1 = Score::new(100);
        let s2 = Score::new(50);

        assert!(s1 > s2);

        // Useful for priority queues where high score = high priority
        let mut scores = [Score::new(30), Score::new(90), Score::new(60)];
        scores.sort();
        // After sort: highest first (because we reversed ordering)
        assert_eq!(scores[0].0, 30);
        assert_eq!(scores[1].0, 60);
        assert_eq!(scores[2].0, 90);
    }
}
