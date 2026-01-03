// --------------------------------

pub fn call_fn<F>(f: F) -> i32
where
    F: Fn() -> i32,
{
    f() + f() // Can be called multiple times
}

pub fn call_fn_mut<F>(mut f: F) -> i32
where
    F: FnMut() -> i32,
{
    f() + f() // Can be called multiple times
}

pub fn call_fn_once<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    f() // Can only be clled once
}

// --------------------------------

pub struct Filter<F>
where
    F: Fn(i32) -> bool,
{
    predicate: F,
}

impl<F> Filter<F>
where
    F: Fn(i32) -> bool,
{
    pub fn new(predicate: F) -> Self {
        Filter { predicate }
    }

    pub fn test(&self, value: i32) -> bool {
        (self.predicate)(value)
    }

    pub fn filter_vec(&self, items: Vec<i32>) -> Vec<i32> {
        items.into_iter().filter(|&x| (self.predicate)(x)).collect()
    }
}

// --------------------------------

pub struct DynamicFilter {
    predicate: Box<dyn Fn(i32) -> bool>,
}

impl DynamicFilter {
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(i32) -> bool + 'static,
    {
        let predicate = Box::new(predicate);
        DynamicFilter { predicate }
    }

    pub fn test(&self, value: i32) -> bool {
        (self.predicate)(value)
    }
}

// --------------------------------

/// Regular function — can be used as Fn
fn is_positive(x: i32) -> bool {
    x > 0
}

/// Function that takes a function pointer (not a closure)
fn apply_fn_ptr(f: fn(i32) -> bool, value: i32) -> bool {
    f(value)
}

/// Function that takes any Fn (closure OR function pointer)
fn apply_fn<F: Fn(i32) -> bool>(f: F, value: i32) -> bool {
    f(value)
}

// --------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_closure() {
        let x = 10;

        // This closure only READS x — implements Fn
        let read_only = || x * 2;

        // Fn closures work with all three functions!
        assert_eq!(call_fn(read_only), 40); // 20 + 20

        let read_only = || x * 2;
        assert_eq!(call_fn_mut(read_only), 40); // Fn implies FnMut

        let read_only = || x * 2;
        assert_eq!(call_fn_once(read_only), 20); // Fn implies FnOnce
    }

    #[test]
    fn test_fn_mut_closure() {
        let mut counter = 0;

        // This closure MUTATES counter — implements FnMut (not Fn)
        let mut increment = || {
            counter += 1;
            counter
        };

        // FnMut works with call_fn_mut and call_fn_once
        assert_eq!(call_fn_mut(&mut increment), 3); // 1 + 2

        // But NOT with call_fn — uncomment to see error:
        // call_fn(increment);  // ERROR: Fn not implemented
    }

    #[test]
    fn test_fn_once_closure() {
        let s = String::from("hello");

        // This closure MOVES s out — implements only FnOnce
        let consume = || {
            let owned = s; // Takes ownership of s
            owned.len() as i32
        };

        // Only works with call_fn_once
        assert_eq!(call_fn_once(consume), 5);

        // Cannot call again — s was moved!
        // call_fn_once(consume);  // ERROR: use of moved value
    }

    #[test]
    fn test_move_keyword() {
        let x = 10;

        // `move` forces the closure to TAKE OWNERSHIP of x
        // But since x is Copy, the closure still implements Fn!
        let moved = move || x * 2;

        assert_eq!(call_fn(moved), 40); // Still works!

        // x is still usable because i32 is Copy
        assert_eq!(x, 10);
    }

    #[test]
    fn test_move_with_non_copy() {
        let s = String::from("world");

        // move with non-Copy type
        let moved = move || s.len() as i32; // s is moved INTO the closure

        // The closure only READS s (now owned), so it's Fn
        assert_eq!(call_fn(moved), 10); // 5 + 5

        // s is no longer accessible here
        // println!("{}", s);  // ERROR: value borrowed after move
    }

    #[test]
    fn test_filter_struct() {
        let is_even = Filter::new(|x| x % 2 == 0);

        assert!(is_even.test(4));
        assert!(!is_even.test(5));

        let evens = is_even.filter_vec(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_with_captured_value() {
        let threshold = 5;
        let above_threshold = Filter::new(move |x| x > threshold);

        assert!(!above_threshold.test(3));
        assert!(above_threshold.test(7));
    }

    #[test]
    fn test_dynamic_filter() {
        // Can store different closures in the same type
        let filters: Vec<DynamicFilter> = vec![
            DynamicFilter::new(|x| x > 0),
            DynamicFilter::new(|x| x % 2 == 0),
            DynamicFilter::new(|x| x < 100),
        ];

        // Test value 50: positive? even? less than 100?
        let results: Vec<bool> = filters.iter().map(|f| f.test(50)).collect();
        assert_eq!(results, vec![true, true, true]);

        // Test value -2: positive? even? less than 100?
        let results: Vec<bool> = filters.iter().map(|f| f.test(-2)).collect();
        assert_eq!(results, vec![false, true, true]);
    }

    #[test]
    fn test_function_pointer() {
        // Function pointer type: fn(i32) -> bool
        let f: fn(i32) -> bool = is_positive;

        assert!(f(5));
        assert!(!f(-3));

        // Works with apply_fn_ptr
        assert!(apply_fn_ptr(is_positive, 10));
    }

    #[test]
    fn test_function_as_fn_trait() {
        // Regular functions implement Fn, FnMut, FnOnce
        assert!(apply_fn(is_positive, 5));

        // Closures also work
        assert!(apply_fn(|x| x < 0, -5));
    }

    #[test]
    fn test_fn_ptr_vs_closure() {
        // Function pointer: fixed size, no captures
        let fn_ptr: fn(i32) -> bool = is_positive;

        // Closure without captures: can coerce to fn pointer!
        let closure_no_capture: fn(i32) -> bool = |x| x > 0;

        assert!(fn_ptr(1));
        assert!(closure_no_capture(1));

        // Closure WITH captures: cannot be fn pointer
        let threshold = 5;
        let closure_captures = |x| x > threshold; // Type is unique, not fn(i32) -> bool

        // This works (generic Fn bound):
        assert!(apply_fn(closure_captures, 10));

        // But this won't compile:
        // let _: fn(i32) -> bool = closure_captures;  // ERROR
    }
}

// --------------------------------
