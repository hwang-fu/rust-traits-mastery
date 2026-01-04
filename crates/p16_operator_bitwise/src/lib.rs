use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

// -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions(u8);

impl Permissions {
    pub const NONE: Self = Permissions(0);
    pub const READ: Self = Permissions(1 << 0); // 0b0001
    pub const WRITE: Self = Permissions(1 << 1); // 0b0010
    pub const EXECUTE: Self = Permissions(1 << 2); // 0b0100
    pub const ALL: Self = Permissions(0b0111);

    /// Check if this permission set contains another
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Check if empty (no permissions)
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// BitOr: Combine permissions (READ | WRITE)
impl BitOr for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Permissions(self.0 | rhs.0)
    }
}

impl BitOr<u8> for Permissions {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        Permissions(self.0 | rhs)
    }
}

// BitAnd: Check/mask permissions (perms & READ)
impl BitAnd for Permissions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Permissions(self.0 & rhs.0)
    }
}

impl BitAnd<u8> for Permissions {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        Permissions(self.0 & rhs)
    }
}

// BitXor: Toggle permissions (perms ^ WRITE)
impl BitXor for Permissions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Permissions(self.0 ^ rhs.0)
    }
}

impl Not for Permissions {
    type Output = Self;

    fn not(self) -> Self::Output {
        // Only invert the bits we care about (lower 3 bits)
        Permissions(!self.0 & 0b0111)
    }
}

impl BitOrAssign for Permissions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Permissions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXorAssign for Permissions {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bits(pub u8);

impl Bits {
    pub fn new(value: u8) -> Self {
        Bits(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

// Shl: Left shift (bits << n)
impl Shl<u8> for Bits {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Bits(self.0 << rhs)
    }
}

// Shr: Right shift (bits >> n)
impl Shr<u8> for Bits {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Bits(self.0 >> rhs)
    }
}

// -----------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_permissions() {
        let perms = Permissions::READ | Permissions::WRITE;

        assert!(perms.contains(Permissions::READ));
        assert!(perms.contains(Permissions::WRITE));
        assert!(!perms.contains(Permissions::EXECUTE));
    }

    #[test]
    fn test_check_permission() {
        let perms = Permissions::READ | Permissions::EXECUTE;

        // Using bitand to check
        assert_eq!(perms & Permissions::READ, Permissions::READ);
        assert_eq!(perms & Permissions::WRITE, Permissions::NONE);
    }

    #[test]
    fn test_toggle_permission() {
        let perms = Permissions::READ | Permissions::WRITE;

        // Toggle WRITE off
        let toggled = perms ^ Permissions::WRITE;
        assert!(toggled.contains(Permissions::READ));
        assert!(!toggled.contains(Permissions::WRITE));

        // Toggle WRITE back on
        let toggled_again = toggled ^ Permissions::WRITE;
        assert_eq!(toggled_again, perms);
    }

    #[test]
    fn test_not() {
        let perms = Permissions::READ;
        let inverted = !perms;

        // Should have WRITE and EXECUTE, but not READ
        assert!(!inverted.contains(Permissions::READ));
        assert!(inverted.contains(Permissions::WRITE));
        assert!(inverted.contains(Permissions::EXECUTE));
    }

    #[test]
    fn test_shift_left() {
        let bits = Bits::new(0b0001);

        assert_eq!((bits << 1).value(), 0b0010);
        assert_eq!((bits << 2).value(), 0b0100);
        assert_eq!((bits << 3).value(), 0b1000);
    }

    #[test]
    fn test_shift_right() {
        let bits = Bits::new(0b1000);

        assert_eq!((bits >> 1).value(), 0b0100);
        assert_eq!((bits >> 2).value(), 0b0010);
        assert_eq!((bits >> 3).value(), 0b0001);
    }

    #[test]
    fn test_shift_create_flags() {
        // Common pattern: create flags by shifting
        let flag_0 = Bits::new(1) << 0; // 0b0001
        let flag_1 = Bits::new(1) << 1; // 0b0010
        let flag_2 = Bits::new(1) << 2; // 0b0100

        assert_eq!(flag_0.value(), 1);
        assert_eq!(flag_1.value(), 2);
        assert_eq!(flag_2.value(), 4);
    }

    #[test]
    fn test_assign_ops() {
        let mut perms = Permissions::READ;

        // Add WRITE
        perms |= Permissions::WRITE;
        assert!(perms.contains(Permissions::READ));
        assert!(perms.contains(Permissions::WRITE));

        // Remove READ using XOR
        perms ^= Permissions::READ;
        assert!(!perms.contains(Permissions::READ));
        assert!(perms.contains(Permissions::WRITE));

        // Mask to keep only WRITE
        perms |= Permissions::EXECUTE;
        perms &= Permissions::WRITE;
        assert_eq!(perms, Permissions::WRITE);
    }
}

// -----------------------------------
