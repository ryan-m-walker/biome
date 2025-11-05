use biome_rowan::SyntaxKind;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet<K: SyntaxKind>([u128; 2], PhantomData<K>);

impl<K: SyntaxKind> TokenSet<K> {
    pub const EMPTY: Self = Self([0; 2], PhantomData);

    pub fn singleton(kind: K) -> Self {
        unsafe { Self::from_raw(kind.to_raw().0) }
    }

    pub const fn union(self, other: Self) -> Self {
        Self(
            [self.0[0] | other.0[0], self.0[1] | other.0[1]],
            PhantomData,
        )
    }

    pub fn contains(&self, kind: K) -> bool {
        let kind = kind.to_raw().0;
        let num = kind as usize;
        match num {
            0..=127 => self.0[0] & mask(kind)[0] != 0,
            _ => self.0[1] & mask(kind)[1] != 0,
        }
    }

    /// Constructs a token set for a single kind from a kind's raw `u16` representation.
    ///
    /// # Safety
    ///
    /// This method is marked unsafe to discourage its usage over using `TokenSet::singleton`.
    /// It exists to support the `token_set` macro in a `const` context.
    #[doc(hidden)]
    pub const unsafe fn from_raw(kind: u16) -> Self {
        Self(mask(kind), PhantomData)
    }
}

const fn mask(kind: u16) -> [u128; 2] {
    let num = kind as usize;
    match num {
        0..=127 => [1u128 << num, 0],
        128..=255 => [0, 1u128 << (num - 128)],
        _ => panic!("Invalid token kind. TokenSet supports kinds 0-255"),
    }
}

/// Utility macro for making a new token set
#[macro_export]
macro_rules! token_set {
    ($($t:expr),*) => {{
            use $crate::TokenSet;
            TokenSet::EMPTY$(.union(unsafe { TokenSet::from_raw($t as u16) }))*
        }};
    ($($t:expr),* ,) => { token_set!($($t),*) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let [left, right] = mask(0);
        assert_eq!(left.count_ones(), 1);
        assert_eq!(left.trailing_zeros(), 0); // Bit at position 0
        assert_eq!(right, 0);

        // Test the maximum bit position in the left half
        let [left, right] = mask(127);
        assert_eq!(left.count_ones(), 1);
        assert_eq!(left.trailing_zeros(), u128::BITS - 1); // Bit 127 = max bit position
        assert_eq!(right, 0);

        let [left, right] = mask(128);
        assert_eq!(left, 0);
        assert_eq!(right.count_ones(), 1);
        assert_eq!(right.trailing_zeros(), 0); // Bit at position 0

        // Test the maximum bit position in the right half
        let [left, right] = mask(255);
        assert_eq!(left, 0);
        assert_eq!(right.count_ones(), 1);
        assert_eq!(right.trailing_zeros(), u128::BITS - 1); // Bit 127 = max bit position
    }

    #[test]
    #[should_panic(expected = "Invalid token kind. TokenSet supports kinds 0-255")]
    fn test_mask_out_of_range() {
        mask(256);
    }
}
