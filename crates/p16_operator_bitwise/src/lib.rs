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
