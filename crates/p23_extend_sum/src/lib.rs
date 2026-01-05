use std::{
    iter::{Product, Sum},
    ops::{Add, Mul},
};

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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Scalar(pub f64);

impl Scalar {
    pub fn new(value: f64) -> Self {
        Scalar(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    /// Additive identity
    pub fn zero() -> Self {
        Scalar(0.0)
    }

    /// Multiplicative identity
    pub fn one() -> Self {
        Scalar(1.0)
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

// Sum for Scalar
impl Sum for Scalar {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Scalar> for Scalar {
    fn sum<I: Iterator<Item = &'a Scalar>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |acc, x| acc + *x)
    }
}

// Product for Scalar
impl Product for Scalar {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        // Start with ONE (multiplicative identity) and multiply all items
        iter.fold(Scalar::one(), |acc, x| acc * x)
    }
}

impl<'a> Product<&'a Scalar> for Scalar {
    fn product<I: Iterator<Item = &'a Scalar>>(iter: I) -> Self {
        iter.fold(Scalar::one(), |acc, x| acc * *x)
    }
}

// -----------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Money {
    cents: i64,
}

impl Money {
    pub fn new(cents: i64) -> Self {
        Money { cents }
    }

    pub fn from_dollars(dollars: i64) -> Self {
        Money {
            cents: dollars * 100,
        }
    }

    pub fn cents(&self) -> i64 {
        self.cents
    }

    pub fn dollars(&self) -> f64 {
        self.cents as f64 / 100.0
    }

    /// The additive identity (zero)
    pub fn zero() -> Self {
        Money { cents: 0 }
    }

    /// The multiplicative identity (one dollar = 100 cents for scaling)
    pub fn one() -> Self {
        Money { cents: 100 }
    }
}

// Add support for Money + Money
impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money {
            cents: self.cents + rhs.cents,
        }
    }
}

// Mul support for Money * i64 (scaling)
impl Mul<i64> for Money {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Money {
            cents: self.cents * rhs,
        }
    }
}

// Sum: enable .sum() on Iterator<Item = Money>
impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Money::zero(), |acc, x| acc + x)
    }
}

// Sum for references: enable .sum() on Iterator<Item = &Money>
impl<'a> Sum<&'a Money> for Money {
    fn sum<I: Iterator<Item = &'a Money>>(iter: I) -> Self {
        iter.fold(Money::zero(), |acc, x| acc + *x)
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

    #[test]
    fn test_money_basic() {
        let m = Money::from_dollars(10);
        assert_eq!(m.cents(), 1000);
        assert_eq!(m.dollars(), 10.0);

        let m2 = Money::new(550);
        assert_eq!(m2.dollars(), 5.5);
    }

    #[test]
    fn test_money_add() {
        let a = Money::from_dollars(10);
        let b = Money::from_dollars(5);
        let c = a + b;

        assert_eq!(c.cents(), 1500);
    }

    #[test]
    fn test_money_sum_owned() {
        let amounts = vec![
            Money::from_dollars(10),
            Money::from_dollars(20),
            Money::from_dollars(15),
        ];

        // .sum() works because we implemented Sum for Money
        let total: Money = amounts.into_iter().sum();
        assert_eq!(total.cents(), 4500); // $45.00
    }

    #[test]
    fn test_money_sum_references() {
        let amounts = [
            Money::from_dollars(5),
            Money::from_dollars(10),
            Money::from_dollars(3),
        ];

        // .sum() on references works because we implemented Sum<&Money>
        let total: Money = amounts.iter().sum();
        assert_eq!(total.cents(), 1800); // $18.00

        // Original vec is still usable
        assert_eq!(amounts.len(), 3);
    }

    #[test]
    fn test_money_sum_empty() {
        let empty: Vec<Money> = vec![];

        // Sum of empty iterator returns the identity (zero)
        let total: Money = empty.into_iter().sum();
        assert_eq!(total.cents(), 0);
    }
}
