use std::ops::{BitAnd, BitAndAssign, BitOr, Not};

#[derive(Clone, Copy, Default)]
pub struct U256 {
    left: u128,
    right: u128,
}

impl U256 {
    pub fn bit_set(&mut self, offset: usize) {
        if offset < 128 {
            self.left |= 1 << (127 - offset);
        } else {
            self.right |= 1 << (255 - offset);
        }
    }

    pub const fn count_ones(&self) -> u32 {
        self.left.count_ones() + self.right.count_ones()
    }

    pub const fn non_zero(&self) -> bool {
        self.left != 0 || self.right != 0
    }

    /// Used to find the bounding rectangle for part one.
    pub const fn min_set(&self) -> Option<u32> {
        if self.left != 0 {
            Some(self.left.leading_zeros())
        } else if self.right != 0 {
            Some(128 + self.right.leading_zeros())
        } else {
            None
        }
    }

    /// Used to find the bounding rectangle for part one.
    pub const fn max_set(&self) -> Option<u32> {
        if self.right != 0 {
            Some(255 - self.right.trailing_zeros())
        } else if self.left != 0 {
            Some(127 - self.left.trailing_zeros())
        } else {
            None
        }
    }

    pub const fn left_shift(&self) -> Self {
        Self {
            left: (self.left << 1) | (self.right >> 127),
            right: (self.right << 1),
        }
    }

    pub const fn right_shift(&self) -> Self {
        Self {
            left: (self.left >> 1),
            right: (self.left << 127) | (self.right >> 1),
        }
    }
}

/// Syntactic sugar to provide the regular `&`, `|` and `!` bitwise operator notation.
impl BitAnd for U256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            left: self.left & rhs.left,
            right: self.right & rhs.right,
        }
    }
}

impl BitOr for U256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            left: self.left | rhs.left,
            right: self.right | rhs.right,
        }
    }
}

impl Not for U256 {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            left: !self.left,
            right: !self.right,
        }
    }
}

impl BitAndAssign for U256 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.left &= rhs.left;
        self.right &= rhs.right;
    }
}
