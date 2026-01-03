// -----------------------------------

pub struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let value = self.start;
            self.start += 1;
            Some(value)
        } else {
            None
        }
    }

    // Override size_hint for ExactSizeIterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end - self.start).max(0) as usize;
        (len, Some(len))
    }
}

impl ExactSizeIterator for Range {
    fn len(&self) -> usize {
        (self.end - self.start).max(0) as usize
    }
}

impl DoubleEndedIterator for Range {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            Some(self.end)
        } else {
            None
        }
    }
}

// -----------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct List<T> {
    items: Vec<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List::new();
        list.extend(iter);
        list
    }
}

// -----------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_basic() {
        let rng = Range::new(1, 5);
        let v: Vec<i32> = rng.collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_exact_size_len() {
        let rng = Range::new(0, 10);
        assert_eq!(rng.len(), 10);

        let rng = Range::new(5, 5);
        assert_eq!(rng.len(), 0);

        let rng = Range::new(10, 5);
        assert_eq!(rng.len(), 0);
    }

    #[test]
    fn test_exact_size_updates() {
        let mut range = Range::new(0, 5);
        assert_eq!(range.len(), 5);

        range.next();
        assert_eq!(range.len(), 4);

        range.next();
        range.next();
        assert_eq!(range.len(), 2);
    }

    #[test]
    fn test_double_ended_next_back() {
        let mut range = Range::new(1, 5);

        assert_eq!(range.next_back(), Some(4));
        assert_eq!(range.next_back(), Some(3));
        assert_eq!(range.next_back(), Some(2));
        assert_eq!(range.next_back(), Some(1));
        assert_eq!(range.next_back(), None);
    }

    #[test]
    fn test_double_ended_rev() {
        // .rev() is enabled by DoubleEndedIterator
        let range = Range::new(1, 5);
        let reversed: Vec<i32> = range.rev().collect();

        assert_eq!(reversed, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_double_ended_mixed() {
        let mut range = Range::new(1, 6); // 1, 2, 3, 4, 5

        assert_eq!(range.next(), Some(1)); // front
        assert_eq!(range.next_back(), Some(5)); // back
        assert_eq!(range.next(), Some(2)); // front
        assert_eq!(range.next_back(), Some(4)); // back
        assert_eq!(range.next(), Some(3)); // front - last item
        assert_eq!(range.next(), None); // exhausted
        assert_eq!(range.next_back(), None); // exhausted
    }

    #[test]
    fn test_extend_basic() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        // Extend with more items
        list.extend(vec![3, 4, 5]);

        let values: Vec<&i32> = list.iter().collect();
        assert_eq!(values, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_extend_from_range() {
        let mut list: List<i32> = List::new();

        // Extend with our custom Range
        list.extend(Range::new(1, 4));

        let values: Vec<&i32> = list.iter().collect();
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_extend_multiple_times() {
        let mut list = List::new();

        list.extend([1, 2]);
        list.extend([3, 4]);
        list.extend(std::iter::once(5));

        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_from_iterator_uses_extend() {
        // FromIterator is often implemented using Extend
        let list: List<i32> = Range::new(0, 5).collect();

        let values: Vec<&i32> = list.iter().collect();
        assert_eq!(values, vec![&0, &1, &2, &3, &4]);
    }
}

// -----------------------------------
