// -----------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct Bag<T> {
    items: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        Bag { items: Vec::new() }
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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> Default for Bag<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Extend: append items from an iterator to the bag
impl<T> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}

// FromIterator: create a Bag from an iterator (uses Extend internally!)
impl<T> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bag = Bag::new();
        bag.extend(iter); // Reuse Extend implementation
        bag
    }
}

// -----------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_extend_basic() {
        let mut bag = Bag::new();
        bag.push(1);
        bag.push(2);
        bag.extend(vec![3, 4, 5]);

        assert_eq!(bag.len(), 5);

        let items: Vec<_> = bag.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bag_extend_from_iterator() {
        let mut bag = Bag::new();
        bag.push("a");

        // Extend from any iterator
        bag.extend(["b", "c", "d"].iter().copied());
        bag.extend(Some("e")); // Option is an iterator!

        assert_eq!(bag.len(), 5);
    }

    #[test]
    fn test_bag_from_iterator() {
        // collect() uses FromIterator, which uses our Extend
        let bag: Bag<i32> = (1..=5).collect();

        assert_eq!(bag.len(), 5);
        let items: Vec<_> = bag.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bag_extend_empty() {
        let mut bag: Bag<i32> = Bag::new();

        // Extending with empty iterator is fine
        bag.extend(std::iter::empty());
        assert_eq!(bag.len(), 0);

        // Extending with items works
        bag.extend([1, 2]);
        assert_eq!(bag.len(), 2);
    }
}
