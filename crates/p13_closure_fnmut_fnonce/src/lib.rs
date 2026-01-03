// -------------------------------------

/// Calls a FnMut closure n times, returning final result
pub fn call_n_times<F>(mut f: F, n: usize) -> i32
where
    F: FnMut() -> i32,
{
    let mut result = 0;
    for _ in 0..n {
        result = f();
    }
    result
}

/// A counter factory — returns a FnMut closure
pub fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

/// Accumulator that sums values
pub fn make_accumulator() -> impl FnMut(i32) -> i32 {
    let mut sum = 0;
    move |x| {
        sum += x;
        sum
    }
}

// -------------------------------------

pub fn run_once<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}

pub fn into_getter<T>(value: T) -> impl FnOnce() -> T {
    move || value
}

pub struct Lazy<T, F>
where
    F: FnOnce() -> T,
{
    init: Option<F>,
    value: Option<T>,
}

impl<T, F> Lazy<T, F>
where
    F: FnOnce() -> T,
{
    pub fn new(init: F) -> Self {
        let init = Some(init);
        let value = None;
        Lazy { init, value }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().expect("Lazy already initializated");
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}

/// Spawns a task that moves ownership into it (simulated)
pub fn spawn_task<F>(task: F) -> String
where
    F: FnOnce() -> String,
{
    // In real code, this might spawn a thread
    task()
}

// -------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_n_times() {
        let mut counter = 0;
        let increment = || {
            counter += 1;
            counter
        };

        let result = call_n_times(increment, 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_make_counter() {
        let mut counter = make_counter(0);

        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_multiple_counters() {
        // Each counter has its own state!
        let mut counter_a = make_counter(0);
        let mut counter_b = make_counter(100);

        assert_eq!(counter_a(), 1);
        assert_eq!(counter_b(), 101);
        assert_eq!(counter_a(), 2);
        assert_eq!(counter_b(), 102);
    }

    #[test]
    fn test_accumulator() {
        let mut acc = make_accumulator();

        assert_eq!(acc(10), 10);
        assert_eq!(acc(5), 15);
        assert_eq!(acc(25), 40);
    }

    #[test]
    fn test_run_once() {
        let s = String::from("hello");

        // Closure takes ownership of s
        let consume = || {
            let owned = s;
            owned.to_uppercase()
        };

        let result = run_once(consume);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_into_getter() {
        let data = vec![1, 2, 3];
        let getter = into_getter(data);

        // Can only call once — data is moved out
        let retrieved = getter();
        assert_eq!(retrieved, vec![1, 2, 3]);

        // Can't call again:
        // let _ = getter();  // ERROR: use of moved value
    }

    #[test]
    fn test_lazy_initialization() {
        use std::cell::Cell;

        let call_count = Cell::new(0);

        let mut lazy = Lazy::new(|| {
            call_count.set(call_count.get() + 1);
            "expensive computation".to_string()
        });

        // Not yet initialized
        assert_eq!(call_count.get(), 0);

        // First access triggers init
        assert_eq!(lazy.get(), "expensive computation");
        assert_eq!(call_count.get(), 1);

        // Second access reuses cached value
        assert_eq!(lazy.get(), "expensive computation");
        assert_eq!(call_count.get(), 1); // Still 1!
    }

    #[test]
    fn test_spawn_task_with_owned_data() {
        let name = String::from("Alice");

        // Move name into the task
        let task = move || format!("Hello, {}!", name);

        let result = spawn_task(task);
        assert_eq!(result, "Hello, Alice!");

        // name is no longer accessible
        // println!("{}", name);  // ERROR
    }
}

// -------------------------------------
