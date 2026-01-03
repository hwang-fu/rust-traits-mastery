// ------------------------------------

#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.items.iter_mut()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

/// This is what powers .collect()
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for item in iter {
            stack.push(item);
        }
        stack
    }
}

// ------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter_consuming() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut sum = 0;
        for n in stack {
            sum += n;
        }
        assert_eq!(sum, 6);

        // stack is no longer usable — it was moved!
        // stack.push(4);  // ERROR: use of moved value
    }

    #[test]
    fn test_into_iter_collect() {
        let mut stack = Stack::new();
        stack.push("a");
        stack.push("b");
        stack.push("c");

        let v: Vec<&str> = stack.into_iter().collect();
        assert_eq!(v, vec!["a", "b", "c"]);

        // stack is no longer usable — it was moved!
        // stack.push("d"); // ERROR: use of moved value
    }

    #[test]
    fn test_iter_borrowing() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        let mut sum = 0;
        for n in &stack {
            sum += n;
        }
        assert_eq!(sum, 60);

        stack.push(40);
        assert_eq!(stack.len(), 4)
    }

    #[test]
    fn test_iter_method() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // here &i32 can be used with * is because the Mul<&i32> was implemented
        let v: Vec<i32> = stack.iter().map(|n| n * 2).collect();
        assert_eq!(v, vec![2, 4, 6]);

        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_iter_mut_borrowing() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        for n in &mut stack {
            (*n) *= 10;
        }

        let v: Vec<i32> = stack.into_iter().collect();
        assert_eq!(v, vec![10, 20, 30]);
    }

    #[test]
    fn test_iter_mut_method() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));

        for s in stack.iter_mut() {
            s.push('!');
        }

        let v: Vec<&String> = stack.iter().collect();
        assert_eq!(v, vec!["hello!", "world!"]);
    }

    #[test]
    fn test_from_iterator_collect() {
        let stack: Stack<i32> = (1..=5).collect();
        let v: Vec<i32> = stack.into_iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_from_iterator_from_vec() {
        let v = vec!["a", "b", "c"];
        let stack: Stack<&str> = v.into_iter().collect();
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_from_iterator_with_map() {
        let stack: Stack<String> = (1..=3).map(|n| format!("item-{}", n)).collect();
        let v: Vec<&String> = stack.iter().collect();
        assert_eq!(v, vec!["item-1", "item-2", "item-3"]);
    }

    // These work because the standard library implements:
    // - FromIterator<Result<T, E>> for Result<Collection<T>, E>
    // - FromIterator<Option<T>> for Option<Collection<T>>

    #[test]
    fn test_collect_into_result() {
        // Each parse returns Result<i32, ParseIntError>
        let strings = ["1", "2", "3"];

        let v: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse()).collect();
        assert!(v.is_ok());
        assert_eq!(v.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_into_result_fails() {
        let strings = ["1", "oops", "3"];

        let v: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse()).collect();
        assert!(v.is_err());
    }

    #[test]
    fn test_collect_into_option() {
        let numbers = [Some(1), Some(2), Some(3)];
        let v: Option<Vec<i32>> = numbers.into_iter().collect();
        assert!(v.is_some());
        assert_eq!(v.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_into_option_fails() {
        let numbers = [Some(1), None, Some(3)];
        let v: Option<Vec<i32>> = numbers.into_iter().collect();
        assert!(v.is_none());
    }
}
