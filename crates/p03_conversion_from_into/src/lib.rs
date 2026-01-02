//! From<T> - Infallible conversion from T to Self
//! Into<T> - Automatically derived from From (never implement directly)

// ------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl Celsius {
    pub fn new(value: f64) -> Self {
        Celsius(value)
    }
}

impl Fahrenheit {
    pub fn new(value: f64) -> Self {
        Fahrenheit(value)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        let c = (f.0 - 32.0) * 5.0 / 9.0;
        Celsius(c)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        let f = c.0 * 9.0 / 5.0 + 32.0;
        Fahrenheit(f)
    }
}

// ------------------------------

pub struct UserId(pub u64);
pub struct OrderId(pub u64);

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

impl From<u64> for OrderId {
    fn from(id: u64) -> Self {
        OrderId(id)
    }
}

impl From<UserId> for u64 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<OrderId> for u64 {
    fn from(id: OrderId) -> Self {
        id.0
    }
}

// ------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Email(pub String);

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s)
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserAccount {
    pub name: String,
    pub email: Email,
}

impl UserAccount {
    pub fn new(name: impl Into<String>, email: impl Into<Email>) -> Self {
        UserAccount {
            name: name.into(),
            email: email.into(),
        }
    }
}

// ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fahrenheit_to_celsius() {
        let f = Fahrenheit::new(32.0);
        let c = Celsius::from(f);
        assert_eq!(c.0, 0.0)
    }

    #[test]
    fn test_from_celsius_to_fahrenheit() {
        let c = Celsius::new(100.0);
        let f = Fahrenheit::from(c);
        assert_eq!(f.0, 212.0);
    }

    #[test]
    fn test_into_is_automatic() {
        let f = Fahrenheit::new(212.0);
        let c: Celsius = f.into();
        assert_eq!(c.0, 100.0);
    }

    #[test]
    fn test_into_with_turbofish() {
        let c = Celsius::new(0.0);
        let f: Fahrenheit = c.into();
        assert_eq!(f.0, 32.0);
    }

    #[test]
    fn test_newtype_from_primitive() {
        // Create IDs from u64
        let user_id = UserId::from(42);
        let order_id = OrderId::from(42);

        // They have the same inner value but are different types!
        assert_eq!(user_id.0, 42);
        assert_eq!(order_id.0, 42);

        // This won't compile - type safety!
        // let wrong: UserId = order_id; // ERROR: mismatched types
    }

    #[test]
    fn test_newtype_into_primitive() {
        let user_id = UserId(100);

        // Extract the inner value using Into
        let raw: u64 = user_id.into();

        assert_eq!(raw, 100);
    }

    #[test]
    fn test_from_str_and_string() {
        // From<&str>
        let email1 = Email::from("alice@example.com");

        // From<String>
        let email2 = Email::from(String::from("bob@example.com"));

        assert_eq!(email1.0, "alice@example.com");
        assert_eq!(email2.0, "bob@example.com");
    }

    #[test]
    fn test_flexible_api_with_into() {
        // All of these work thanks to impl Into<Email>!

        // Pass &str directly
        let user1 = UserAccount::new("Alice", "alice@example.com");

        // Pass String
        let user2 = UserAccount::new(String::from("Bob"), String::from("bob@example.com"));

        // Pass Email directly
        let user3 = UserAccount::new("Charlie", Email::from("charlie@example.com"));

        assert_eq!(user1.email.0, "alice@example.com");
        assert_eq!(user2.email.0, "bob@example.com");
        assert_eq!(user3.email.0, "charlie@example.com");
    }

    #[test]
    fn test_reflexive_from() {
        // Every type implements From<T> for T (identity conversion)
        let email = Email::from("test@example.com");

        // This works because of the blanket impl: From<Email> for Email
        let same_email = Email::from(email.clone());

        assert_eq!(email, same_email);

        // This is why impl Into<T> works so flexibly -
        // you can always pass T directly where Into<T> is expected
    }
}
