// ----------------------

pub struct Counter {
    current: u32,
    maximum: u32,
}

impl Counter {
    pub fn new(maximum: u32) -> Self {
        let current = 0;
        Counter { current, maximum }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current <= self.maximum {
            Some(self.current)
        } else {
            None
        }
    }
}

// ----------------------

pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;

        self.current = self.next;
        self.next += item;

        Some(item) // Always returns Some — infinite iterator!
    }
}

// ----------------------

pub struct Cycle<T> {
    items: Vec<T>,
}

impl<T> Cycle<T> {
    pub fn new(items: Vec<T>) -> Self {
        Cycle { items }
    }

    pub fn iter(&self) -> CycleIter<'_, T> {
        CycleIter {
            cycle: self,
            index: 0,
        }
    }
}

pub struct CycleIter<'a, T> {
    cycle: &'a Cycle<T>,
    index: usize,
}

impl<'a, T> Iterator for CycleIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle.items.is_empty() {
            return None;
        }

        let item = &self.cycle.items[self.index];
        self.index += 1;
        self.index %= self.cycle.items.len();
        Some(item)
    }
}

// ----------------------

#[cfg(test)]
mod tests {
    use core::str;
    use std::usize;

    use super::*;

    #[test]
    fn test_counter_basic() {
        let mut counter = Counter::new(3);

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_counter_with_for_loop() {
        let mut sum = 0;

        for n in Counter::new(5) {
            sum += n;
        }

        assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
    }

    #[test]
    fn test_counter_collect() {
        // You didn't implement collect(), sum(), or for-loop support — they all work because you implemented Iterator!
        let v: Vec<u32> = Counter::new(4).collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_counter_sum() {
        // You didn't implement collect(), sum(), or for-loop support — they all work because you implemented Iterator!
        let total: u32 = Counter::new(10).sum();
        assert_eq!(total, 55);
    }

    #[test]
    fn test_map_adapter() {
        let v: Vec<u32> = Counter::new(4).map(|n| n * 2).collect();
        assert_eq!(v, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_adapter() {
        let v: Vec<u32> = Counter::new(10).filter(|n| n % 2 == 0).collect();
        assert_eq!(v, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_take_adapter() {
        let v: Vec<u32> = Counter::new(100).take(3).collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_skip_adapter() {
        let v: Vec<u32> = Counter::new(5).skip(2).collect();
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn test_chained_adapters() {
        let n: u32 = Counter::new(10).skip(2).take(3).map(|n| n * 2).sum();
        assert_eq!(n, 24);
    }

    #[test]
    fn test_find() {
        let first_even_number = Counter::new(10).find(|&n| n % 2 == 0);
        assert!(first_even_number.is_some());
        assert_eq!(first_even_number.unwrap(), 2);

        let first_number_greater_than_100 = Counter::new(10).find(|n| n > &100);
        assert!(first_number_greater_than_100.is_none());
    }

    #[test]
    fn test_any_all() {
        assert!(Counter::new(10).any(|n| n % 2 == 0));
        assert!(!Counter::new(10).any(|n| n > 100));

        assert!(Counter::new(10).all(|n| n > 0));
        assert!(!Counter::new(10).all(|n| n == 1));
    }

    #[test]
    fn test_position() {
        let pos = Counter::new(10).position(|n| n == 5);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap(), 4); // 0-indexed
    }

    #[test]
    fn test_fold() {
        let total = Counter::new(5).fold(0, |acc, curr| acc + curr);
        assert_eq!(total, 15);

        let product = Counter::new(5).fold(1, |acc, n| acc * n);
        assert_eq!(product, 120);

        let s = Counter::new(3).fold(String::new(), |mut acc, n| {
            if !acc.is_empty() {
                acc.push('-');
            }
            acc.push_str(&n.to_string());
            acc
        });
        assert_eq!(s, "1-2-3".to_string());
    }

    #[test]
    fn test_fibonacci_first_ten() {
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_sum() {
        let sum: u64 = Fibonacci::new().take(10).sum();
        assert_eq!(sum, 88);
    }

    #[test]
    fn test_fibonacci_filter() {
        let even_fibs: Vec<u64> = Fibonacci::new().filter(|&n| n % 2 == 0).take(5).collect();
        assert_eq!(even_fibs, vec![0, 2, 8, 34, 144]);
    }

    #[test]
    fn test_cycle_basic() {
        let cycle = Cycle::new(vec!["a", "b", "c"]);
        let items: Vec<&str> = cycle.iter().take(7).copied().collect();
        assert_eq!(items, vec!["a", "b", "c", "a", "b", "c", "a"]);
    }

    #[test]
    fn test_cycle_empty() {
        let cycle: Cycle<i32> = Cycle::new(vec![]);
        let items: Vec<&i32> = cycle.iter().take(5).collect();
        assert_eq!(items, Vec::<&i32>::new()); // Empty!
    }

    #[test]
    fn test_cycle_single() {
        let cycle = Cycle::new(vec![42]);
        let items: Vec<&i32> = cycle.iter().take(3).collect();
        assert_eq!(items, vec![&42, &42, &42]);
    }

    #[test]
    fn test_enumerate() {
        let v: Vec<(usize, u32)> = Counter::new(3).enumerate().collect();
        assert_eq!(v, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_zip() {
        let xs = vec!['a', 'b', 'c'];
        let ys = Counter::new(3);
        let v: Vec<(char, u32)> = xs.into_iter().zip(ys).collect();
        assert_eq!(v, vec![('a', 1), ('b', 2), ('c', 3)]);
    }

    #[test]
    fn test_zip_unequal_lengths() {
        let xs = vec![1, 2];
        let ys = Counter::new(100);
        let v: Vec<(i32, u32)> = xs.into_iter().zip(ys).collect();
        assert_eq!(v, vec![(1, 1), (2, 2)]); // Only 2 pairs!
    }
}
