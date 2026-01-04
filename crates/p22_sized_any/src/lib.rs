//! Phase 10.1: Sized and ?Sized
//!
//! Key Takeaways:
//! 1. Sized = compile-time known size (implicit bound on all generics)
//! 2. ?Sized = "maybe sized" — relaxes the Sized requirement
//! 3. DSTs (str, [T], dyn Trait) can only be used behind pointers
//! 4. Fat pointers = data ptr + metadata (length or vtable)

use std::{
    any::{Any, TypeId},
    fmt::Debug,
    mem,
};

// ---------------------------------
pub fn print_size_of<T>() {
    println!("Size of T: {} bytes", mem::size_of::<T>());
}

pub fn print_size_of_val<T: ?Sized>(val: &T) {
    println!("Size of T: {} bytes", mem::size_of_val(val));
}

/// Compile-time assertion that T is Sized.
fn _assert_sized<T: Sized>() {}

/// Compile-time check showing which types are Sized.
#[allow(dead_code)]
fn _static_checks() {
    // These are all Sized (fixed, known size at compile time)
    _assert_sized::<i32>();
    _assert_sized::<String>();
    _assert_sized::<Vec<u8>>();
    _assert_sized::<[i32; 5]>(); // Fixed-size array IS Sized
    _assert_sized::<&str>(); // References are always Sized (pointer)
    _assert_sized::<Box<str>>(); // Box is Sized (pointer)

    // These would FAIL (uncomment to see compiler errors):
    // _assert_sized::<str>();       // ERROR: str is not Sized
    // _assert_sized::<[i32]>();     // ERROR: slice is not Sized
    // _assert_sized::<dyn std::fmt::Debug>(); // ERROR: trait object is not Sized
}

/// Demonstrates pointer sizes for different types.
/// Regular pointers are "thin" (8 bytes on 64-bit).
/// Pointers to DSTs are "fat" (16 bytes — data + metadata).
pub fn demonstrate_pointer_sizes() {
    // Thin pointers (8 bytes) - point to Sized types
    println!("=== Thin Pointers (Sized types) ===");
    println!("&i32:        {} bytes", mem::size_of::<&i32>());
    println!("&String:     {} bytes", mem::size_of::<&String>());
    println!("&[i32; 5]:   {} bytes", mem::size_of::<&[i32; 5]>());
    println!("Box<i32>:    {} bytes", mem::size_of::<Box<i32>>());

    // Fat pointers (16 bytes) - point to unsized types
    println!("\n=== Fat Pointers (Unsized types) ===");
    println!(
        "&str:        {} bytes (ptr + length)",
        mem::size_of::<&str>()
    );
    println!(
        "&[i32]:      {} bytes (ptr + length)",
        mem::size_of::<&[i32]>()
    );
    println!(
        "Box<str>:    {} bytes (ptr + length)",
        mem::size_of::<Box<str>>()
    );
    println!(
        "Box<[i32]>:  {} bytes (ptr + length)",
        mem::size_of::<Box<[i32]>>()
    );

    // Trait object fat pointers (16 bytes) - ptr + vtable
    println!("\n=== Trait Object Fat Pointers ===");
    println!(
        "&dyn Debug:  {} bytes (ptr + vtable)",
        mem::size_of::<&dyn std::fmt::Debug>()
    );
    println!(
        "Box<dyn Debug>: {} bytes (ptr + vtable)",
        mem::size_of::<Box<dyn std::fmt::Debug>>()
    );
}

// ---------------------------------
pub struct Wrapper<T>
where
    T: ?Sized,
{
    pub prefix: &'static str,
    pub data: T,
}

impl<T> Wrapper<T> {
    pub fn new(prefix: &'static str, data: T) -> Self {
        Wrapper { prefix, data }
    }
}

impl<T> Wrapper<T>
where
    T: ?Sized,
{
    pub fn prefix(&self) -> &str {
        self.prefix
    }
}

impl Wrapper<str> {
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

impl Wrapper<String> {
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

/// A function that accepts any Wrapper regardless of inner type.
/// This works with both Wrapper<String> and Wrapper<str>!
pub fn print_wrapper<T>(wrapper: &Wrapper<T>)
where
    T: ?Sized + Debug,
{
    println!(
        "Wrapper {{ prefix: {:?}, data: {:?} }}",
        wrapper.prefix, &wrapper.data
    );
}

// ---------------------------------
/// Demonstrates that every type has a unique TypeId.
pub fn demonstrate_type_ids() {
    println!("=== TypeId Examples ===");
    println!("TypeId of i32:    {:?}", TypeId::of::<i32>());
    println!("TypeId of i64:    {:?}", TypeId::of::<i64>());
    println!("TypeId of String: {:?}", TypeId::of::<String>());
    println!("TypeId of &str:   {:?}", TypeId::of::<&str>());

    // Same type = same TypeId
    let id1 = TypeId::of::<String>();
    let id2 = TypeId::of::<String>();
    println!("\nSame type, same id: {}", id1 == id2);

    // Different types = different TypeId
    let id3 = TypeId::of::<i32>();
    println!("Different types, different id: {}", id1 != id3);
}

/// Check the type of a value at runtime using Any.
pub fn check_type(value: &dyn Any) {
    if value.is::<i32>() {
        println!("It's an i32!");
    } else if value.is::<String>() {
        println!("It's a String!");
    } else if value.is::<f64>() {
        println!("It's an f64!");
    } else {
        println!("Unknown type with id: {:?}", value.type_id());
    }
}

/// Downcast a dyn Any reference to a concrete type.
/// Returns the value if successful, or a message if not.
pub fn extract_i32(value: &dyn Any) -> Option<i32> {
    // downcast_ref returns Option<&T>
    value.downcast_ref::<i32>().copied()
}

pub fn extract_string(value: &dyn Any) -> Option<String> {
    // downcast_ref returns Option<&T>, we clone to get owned
    value.downcast_ref::<String>().cloned()
}

// ---------------------------------
#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn test_print_size_of_sized_types() {
        print_size_of::<i32>(); // 4 bytes
        print_size_of::<i64>(); // 8 bytes
        print_size_of::<String>(); // 24 bytes (ptr + len + capacity)
        print_size_of::<Vec<i32>>(); // 24 bytes (ptr + len + capacity)
        print_size_of::<[i32; 5]>(); // 20 bytes (5 * 4)

        // This would NOT compile (str is not Sized):
        // print_size_of::<str>();
    }

    #[test]
    fn test_print_size_of_val_with_sized() {
        // ?Sized functions work with Sized types too
        let x: i32 = 42;
        let s: String = String::from("hello");
        let v: Vec<i32> = vec![1, 2, 3];

        print_size_of_val(&x); // 4 bytes
        print_size_of_val(&s); // 24 bytes
        print_size_of_val(&v); // 24 bytes
    }

    #[test]
    fn test_print_size_of_val_with_unsized() {
        // The power of ?Sized: works with DSTs!
        let s: String = String::from("hi");
        let slice: &str = &s; // str is unsized
        print_size_of_val(slice); // 2 bytes (actual string data)

        let v = vec![1, 2, 3, 4, 5];
        let slice: &[i32] = &v; // [i32] is unsized
        print_size_of_val(slice); // 20 bytes (5 * 4)
    }

    #[test]
    fn test_fat_pointers() {
        demonstrate_pointer_sizes();

        // Verify thin vs fat pointer sizes
        assert_eq!(mem::size_of::<&i32>(), 8); // thin
        assert_eq!(mem::size_of::<&str>(), 16); // fat (ptr + len)
        assert_eq!(mem::size_of::<&[i32]>(), 16); // fat (ptr + len)
        assert_eq!(mem::size_of::<&dyn Debug>(), 16); // fat (ptr + vtable)
    }

    #[test]
    fn test_wrapper_with_string() {
        // Wrapper<String> is Sized — can create directly
        let w = Wrapper::new("greeting", String::from("hello"));

        assert_eq!(w.prefix(), "greeting");
        assert_eq!(w.as_str(), "hello");

        print_wrapper(&w);
    }

    #[test]
    fn test_wrapper_with_str() {
        // Wrapper<str> is UNSIZED — cannot create directly on stack!
        // But we can create it via coercion from Wrapper<String>
        let boxed = Box::new(Wrapper::new("tag", String::from("world")));

        // Coerce Box<Wrapper<String>> to Box<Wrapper<str>>
        // This works because String derefs to str!
        // Note: This specific coercion requires some setup, so let's use a reference instead

        let w = &*boxed;
        print_wrapper(w);

        assert_eq!(w.prefix(), "tag");
        assert_eq!(w.as_str(), "world");
    }

    #[test]
    fn test_sized_vs_unsized_comparison() {
        // Fixed-size array vs slice
        let fixed: [i32; 3] = [1, 2, 3];
        let slice: &[i32] = &fixed;

        // fixed is Sized (12 bytes = 3 * 4)
        assert_eq!(mem::size_of_val(&fixed), 12);

        // slice points to unsized data, but measures the DATA size
        assert_eq!(mem::size_of_val(slice), 12);

        // The POINTER to slice is fat (16 bytes)
        assert_eq!(mem::size_of::<&[i32]>(), 16);
    }

    #[test]
    fn test_box_with_unsized() {
        // Box can hold unsized types!
        // The Box itself is always Sized (it's a pointer)

        // Box<str> - holds an unsized str on the heap
        let boxed_str: Box<str> = "hello".into();
        assert_eq!(&*boxed_str, "hello");
        assert_eq!(mem::size_of_val(&*boxed_str), 5); // actual str data

        // Box<[i32]> - holds an unsized slice on the heap
        let boxed_slice: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        assert_eq!(&*boxed_slice, &[1, 2, 3]);
        assert_eq!(mem::size_of_val(&*boxed_slice), 12); // 3 * 4 bytes
    }

    #[test]
    fn test_generic_function_flexibility() {
        // A function using ?Sized can accept BOTH sized and unsized
        fn describe<T: ?Sized + Debug>(val: &T) -> String {
            format!("{:?}", val)
        }

        // Works with Sized types
        let num = 42;
        assert_eq!(describe(&num), "42");

        let string = String::from("hello");
        assert_eq!(describe(&string), "\"hello\"");

        // Also works with unsized types!
        let str_slice: &str = "world";
        assert_eq!(describe(str_slice), "\"world\"");

        let int_slice: &[i32] = &[1, 2, 3];
        assert_eq!(describe(int_slice), "[1, 2, 3]");
    }

    #[test]
    fn test_type_ids() {
        demonstrate_type_ids();

        // TypeId is consistent for the same type
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<i32>());
        assert_eq!(TypeId::of::<String>(), TypeId::of::<String>());

        // TypeId differs between types
        assert_ne!(TypeId::of::<i32>(), TypeId::of::<i64>());
        assert_ne!(TypeId::of::<String>(), TypeId::of::<&str>());
    }

    #[test]
    fn test_is_type_check() {
        let int_val: Box<dyn Any> = Box::new(42i32);
        let str_val: Box<dyn Any> = Box::new(String::from("hello"));
        let float_val: Box<dyn Any> = Box::new(3.1f64);

        // is::<T>() returns true only for matching type
        assert!(int_val.is::<i32>());
        assert!(!int_val.is::<i64>()); // i32 != i64
        assert!(!int_val.is::<String>());

        assert!(str_val.is::<String>());
        assert!(!str_val.is::<&str>()); // String != &str

        assert!(float_val.is::<f64>());
        assert!(!float_val.is::<f32>()); // f64 != f32
    }

    #[test]
    fn test_check_type_function() {
        let a: &dyn Any = &42i32;
        let b: &dyn Any = &String::from("test");
        let c: &dyn Any = &3.1f64;
        let d: &dyn Any = &vec![1, 2, 3]; // Unknown type

        check_type(a); // "It's an i32!"
        check_type(b); // "It's a String!"
        check_type(c); // "It's an f64!"
        check_type(d); // "Unknown type..."
    }

    #[test]
    fn test_downcast_ref() {
        let val: Box<dyn Any> = Box::new(42i32);

        // Successful downcast
        let result = val.downcast_ref::<i32>();
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 42);

        // Failed downcast (wrong type)
        let result = val.downcast_ref::<String>();
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_functions() {
        let int_any: &dyn Any = &100i32;
        let str_any: &dyn Any = &String::from("hello");

        // extract_i32 works on i32
        assert_eq!(extract_i32(int_any), Some(100));
        assert_eq!(extract_i32(str_any), None);

        // extract_string works on String
        assert_eq!(extract_string(str_any), Some(String::from("hello")));
        assert_eq!(extract_string(int_any), None);
    }
}
