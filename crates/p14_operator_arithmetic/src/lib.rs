use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

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
}
