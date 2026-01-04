use std::ops::{BitAnd, BitOr, BitXor};

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

impl BitXor for Permissions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Permissions(self.0 ^ rhs.0)
    }
}
