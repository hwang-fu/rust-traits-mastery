use std::{cell::Cell, fmt};

pub struct Book {
    pub title: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: &str, pages: u32) -> Self {
        let title = String::from(title);
        Book { title, pages }
    }
}

#[derive(Clone)]
pub struct Magazine {
    pub name: String,
    pub issue: u32,
}

impl Magazine {
    pub fn new(name: &str, issue: u32) -> Self {
        let name = String::from(name);
        Magazine { name, issue }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

/// This struct CANNOT be Copy because String is not Copy.
/// Uncomment the Copy derive to see the compiler error!
#[derive(Debug, Clone)] // Try adding Copy here to see the error
pub struct Person {
    pub name: String, // String is heap-allocated, not Copy
    pub age: u8,      // u8 is Copy
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person {
            name: String::from(name),
            age,
        }
    }
}

/// This struct CAN be Copy because all fields are Copy.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub struct TrackedDocument {
    pub content: String,
    pub clone_count: Cell<u32>,
}

impl TrackedDocument {
    pub fn new(content: &str) -> Self {
        let content = String::from(content);
        let clone_count = Cell::new(0);
        TrackedDocument {
            content,
            clone_count,
        }
    }

    pub fn times_cloned(&self) -> u32 {
        self.clone_count.get()
    }
}

impl Clone for TrackedDocument {
    fn clone(&self) -> Self {
        self.clone_count.set(self.clone_count.get() + 1);

        let content = self.content.clone();
        let clone_count = Cell::new(0);
        TrackedDocument {
            content,
            clone_count,
        }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_conn: u32,
}

impl ServerConfig {
    pub fn new(host: &str, port: u16, max_conn: u32) -> Self {
        let host = String::from(host);
        ServerConfig {
            host,
            port,
            max_conn,
        }
    }
}

/// A struct with manual Debug implementation.
/// We'll hide the password field in debug output!
pub struct UserCredentials {
    pub username: String,
    pub password: String, // Sensitive - DONOT show in debug!
}

impl UserCredentials {
    pub fn new(username: &str, password: &str) -> Self {
        UserCredentials {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

impl fmt::Debug for UserCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserCredentials")
            .field("username", &self.username)
            .field("password", &"[SECRET]")
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct AppSettings {
    pub debug_mode: bool, // Default: false
    pub log_level: u8,    // Default: 0
    pub app_name: String, // Default: "" (empty string)
}

pub struct DatabaseConf {
    pub host: String,
    pub port: u16,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

impl Default for DatabaseConf {
    fn default() -> Self {
        let host = String::from("localhost");
        let port = 5432;
        let pool_size = 10;
        let timeout_seconds = 30;
        DatabaseConf {
            host,
            port,
            pool_size,
            timeout_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_semantics() {
        let book1 = Book::new("Rust Programming", 500);

        // This MOVES ownership from book1 to book2
        let book2 = book1;

        // book1 is no longer valid after the move!
        // Uncommenting the next line would cause a compile error:
        // println!("book1 title: {}", book1.title);

        // book2 now owns the data
        assert_eq!(book2.title, "Rust Programming");
        assert_eq!(book2.pages, 500);
    }

    #[test]
    fn test_clone_trait() {
        let mag1 = Magazine::new("Rust Weekly", 42);

        // .clone() creates an explicit copy - both remain valid
        let mag2 = mag1.clone();

        // Both mag1 and mag2 are valid!
        assert_eq!(mag1.name, "Rust Weekly");
        assert_eq!(mag2.name, "Rust Weekly");

        // They are separate copies (modifying one doesn't affect the other)
        // Note: We can't modify here since fields aren't mut, but conceptually
        // mag1 and mag2 own independent data on the heap
    }

    #[test]
    fn test_copy_trait() {
        let p1 = Point::new(10, 20);

        // Copy types are IMPLICITLY copied (no .clone() needed)
        let p2 = p1;

        // Both p1 and p2 are valid! No move occurred.
        assert_eq!(p1.x, 10);
        assert_eq!(p2.x, 10);

        // p1 and p2 are independent copies
        let mut p3 = p1;
        p3.x = 999;

        // p1 is unaffected
        assert_eq!(p1.x, 10);
        assert_eq!(p3.x, 999);
    }

    #[test]
    fn test_person_must_clone() {
        let person1 = Person::new("Alice", 30);

        // Person has String, so it must be cloned explicitly
        let person2 = person1.clone();

        assert_eq!(person1.name, "Alice");
        assert_eq!(person2.name, "Alice");
    }

    #[test]
    fn test_rectangle_is_copy() {
        let rect1 = Rectangle::new(10, 20);

        // Rectangle is Copy, so assignment copies implicitly
        let rect2 = rect1;

        // Both are valid
        assert_eq!(rect1.area(), 200);
        assert_eq!(rect2.area(), 200);
    }

    #[test]
    fn test_manual_clone() {
        let doc1 = TrackedDocument::new("Rust");
        assert_eq!(doc1.times_cloned(), 0);

        let _doc2 = doc1.clone();
        assert_eq!(doc1.times_cloned(), 1);

        let _doc3 = doc1.clone();
        assert_eq!(doc1.times_cloned(), 2);

        let doc4 = doc1.clone();
        assert_eq!(doc4.times_cloned(), 0);
        assert_eq!(doc1.times_cloned(), 3);
    }

    #[test]
    fn test_debug_device() {
        let conf = ServerConfig::new("localhost", 8080, 1000);

        // {:?} gives compact debug output
        let debug_str = format!("{:?}", conf);
        assert!(debug_str.contains("localhost"));
        assert!(debug_str.contains("8080"));

        // {:#?} gives pretty-printed output (useful for nested structs)
        let pretty_debug_str = format!("{:#?}", conf);
        assert!(pretty_debug_str.contains("ServerConfig"));
    }

    #[test]
    fn test_debug_manual() {
        let creds = UserCredentials::new("admin", "admin123");

        let debug_str = format!("{:?}", creds);

        // Username should be visible
        assert!(debug_str.contains("admin"));

        // Password should be hidden!
        assert!(!debug_str.contains("admin123"));
        assert!(debug_str.contains("[SECRET]"));

        println!("{:?}", creds);
        println!("{:#?}", creds);
    }

    #[test]
    fn test_default_derive() {
        let settings = AppSettings::default();

        assert!(!settings.debug_mode);
        assert_eq!(settings.log_level, 0);
        assert_eq!(settings.app_name, "");
    }

    #[test]
    fn test_default_manual() {
        let db_conf = DatabaseConf::default();

        assert_eq!(db_conf.host, "localhost");
        assert_eq!(db_conf.port, 5432);
        assert_eq!(db_conf.pool_size, 10);
        assert_eq!(db_conf.timeout_seconds, 30);
    }

    #[test]
    fn test_default_struct_update() {
        // Only override the fields we care about, rest from Default
        let custom_db = DatabaseConf {
            host: String::from("production.db.example.com"),
            port: 5433,
            ..Default::default() // pool_size=10, timeout_seconds=30
        };

        assert_eq!(custom_db.host, "production.db.example.com");
        assert_eq!(custom_db.port, 5433);
        assert_eq!(custom_db.pool_size, 10); // from Default
        assert_eq!(custom_db.timeout_seconds, 30); // from Default
    }
}
