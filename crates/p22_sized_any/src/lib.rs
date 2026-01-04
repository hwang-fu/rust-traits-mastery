//! Phase 10.1: Sized and ?Sized
//!
//! Key Takeaways:
//! 1. Sized = compile-time known size (implicit bound on all generics)
//! 2. ?Sized = "maybe sized" — relaxes the Sized requirement
//! 3. DSTs (str, [T], dyn Trait) can only be used behind pointers
//! 4. Fat pointers = data ptr + metadata (length or vtable)

use std::mem;

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

// ---------------------------------
#[cfg(test)]
mod tests {
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
}
