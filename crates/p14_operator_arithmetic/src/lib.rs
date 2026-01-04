use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// --------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    /// Zero vector
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    /// Dot product (uses our Mul internally, but returns scalar)
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Length (magnitude) of the vector
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Normalize to unit length
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }
}

// Vec2 + Vec2
impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// &Vec2 + &Vec2
impl Add for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Vec2 + &Vec2
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// &Vec2 + Vec2
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Vec2 - Vec2
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Vec2 * Vec2
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

// Vec2 / Vec2 (element-wise)
impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// Vec2 % Vec2 (element-wise)
impl Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

// -Vec2
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Scalar multiplication: Vec2 * f64
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Scalar multiplication: f64 * Vec2 (for commutativity)
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Self::Output {
        Vec2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

// Scalar division: Vec2 / f64
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Self::Output {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_creation() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let c = a + b;

        assert_eq!(c, Vec2::new(4.0, 6.0));
        assert_eq!(a.x, 1.0);
        assert_eq!(b.x, 3.0);
    }

    #[test]
    fn test_add_references() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        #[allow(clippy::op_ref)]
        let c = &a + &b;

        assert_eq!(c, Vec2::new(4.0, 6.0));
        assert_eq!(a, Vec2::new(1.0, 2.0));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_add_mixed() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let r1 = a + b; // Vec2 + Vec2
        let r2 = &a + &b; // &Vec2 + &Vec2
        let r3 = a + &b; // Vec2 + &Vec2
        let r4 = &a + b; // &Vec2 + Vec2

        assert_eq!(r1, Vec2::new(4.0, 6.0));
        assert_eq!(r2, Vec2::new(4.0, 6.0));
        assert_eq!(r3, Vec2::new(4.0, 6.0));
        assert_eq!(r4, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        assert_eq!(a - b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_mul() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(4.0, 5.0);
        assert_eq!(a * b, Vec2::new(8.0, 15.0));
    }

    #[test]
    fn test_div() {
        let a = Vec2::new(10.0, 20.0);
        let b = Vec2::new(2.0, 4.0);
        assert_eq!(a / b, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_rem() {
        let a = Vec2::new(10.0, 17.0);
        let b = Vec2::new(3.0, 5.0);
        assert_eq!(a % b, Vec2::new(1.0, 2.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec2::new(3.0, -4.0);
        assert_eq!(-v, Vec2::new(-3.0, 4.0));

        // Double negation
        assert_eq!(-(-v), v);
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec2::new(2.0, 3.0);

        // Vec2 * f64
        assert_eq!(v * 2.0, Vec2::new(4.0, 6.0));

        // f64 * Vec2 (commutative)
        assert_eq!(2.0 * v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_scalar_div() {
        let v = Vec2::new(10.0, 20.0);
        assert_eq!(v / 2.0, Vec2::new(5.0, 10.0));
    }

    #[test]
    fn test_assign_ops() {
        let mut v = Vec2::new(1.0, 2.0);

        v += Vec2::new(3.0, 4.0);
        assert_eq!(v, Vec2::new(4.0, 6.0));

        v -= Vec2::new(1.0, 1.0);
        assert_eq!(v, Vec2::new(3.0, 5.0));

        v *= 2.0;
        assert_eq!(v, Vec2::new(6.0, 10.0));

        v /= 2.0;
        assert_eq!(v, Vec2::new(3.0, 5.0));
    }

    #[test]
    fn test_vector_math() {
        let v = Vec2::new(3.0, 4.0);

        // Length of (3, 4) = 5 (Pythagorean triple)
        assert_eq!(v.length(), 5.0);

        // Normalized vector has length 1
        let unit = v.normalize();
        assert!((unit.length() - 1.0).abs() < 1e-10);

        // Dot product
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_eq!(a.dot(b), 0.0); // Perpendicular vectors

        // Combining operations
        let result = (a + b) * 2.0 - Vec2::new(1.0, 1.0);
        assert_eq!(result, Vec2::new(1.0, 1.0));
    }
}
