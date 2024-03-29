#![allow(clippy::unnecessary_cast)]

use crate::Value;

pub const WORD_SIZE: usize = std::mem::size_of::<Value>();
pub const WORD_BIT_SIZE: usize = WORD_SIZE * 8;

pub struct BinComb {
    cur: usize,
    n: usize,
}

impl BinComb {
    pub fn new(n: usize, k: usize) -> Self {
        BinComb {
            cur: (1 << k) - 1,
            n,
        }
    }
}

impl Iterator for BinComb {
    type Item = usize;

    // https://stackoverflow.com/questions/46023719/what-is-an-efficient-code-for-generating-n-binary-digit-numbers-with-k-bits-set
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= (1 << self.n) {
            return None;
        }

        let old = self.cur;

        let lowbit = self.cur & !(self.cur - 1);
        let ones = self.cur & !(self.cur + lowbit);
        self.cur = self.cur + lowbit + ((ones / lowbit) >> 1);

        Some(old)
    }
}

#[inline]
pub fn is_pow2(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[inline]
pub fn pow2(n: usize) -> usize {
    1 << n
}

#[inline]
pub fn halving_mask(i: usize) -> Value {
    let mask: u128 = match i {
        0 => 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA,
        1 => 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC,
        2 => 0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0,
        3 => 0xFF00_FF00_FF00_FF00_FF00_FF00_FF00_FF00,
        4 => 0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000,
        5 => 0xFFFF_FFFF_0000_0000_FFFF_FFFF_0000_0000,
        6 => 0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0000,
        _ => panic!("Unexpected i for halving const"),
    };

    (mask & (Value::MAX as u128)) as Value
}

/// Returns floor(log2(n))
#[inline]
pub fn log2(mut n: usize) -> usize {
    if n <= 1 {
        return 0;
    }

    let mut result = 0;
    while n > 1 {
        n >>= 1;
        result += 1;
    }
    result
}

/// Divides n by `WORD_BIT_SIZE` and ceils result
#[inline]
pub fn div_ws_ceil(n: usize) -> usize {
    (n + (WORD_BIT_SIZE - 1)) >> log2(WORD_BIT_SIZE)
}

/// Divides n by `WORD_BIT_SIZE`
#[inline]
pub fn div_ws(n: usize) -> usize {
    n >> log2(WORD_BIT_SIZE)
}

/// Returns n modulo `WORD_BIT_SIZE`
#[inline]
pub fn mod_ws(n: usize) -> usize {
    n & (WORD_BIT_SIZE - 1)
}

pub fn comb(n: usize, mut r: usize) -> usize {
    // because C(n, r) == C(n, n - r)
    if r > n - r {
        r = n - r;
    }

    let mut ans = 1;

    for i in 1..=r {
        ans *= n - r + i;
        ans /= i;
    }

    ans
}

/// Calculates weight of a factor
#[inline]
pub fn weight(mut n: usize) -> usize {
    let mut weight = 0;
    while n != 0 {
        n = n & (n - 1);
        weight += 1;
    }
    weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow2_works() {
        assert_eq!(pow2(0), 1);
        assert_eq!(pow2(1), 2);
        assert_eq!(pow2(2), 4);
        assert_eq!(pow2(3), 8);
    }

    #[test]
    fn is_pow2_works() {
        assert!(is_pow2(1));
        assert!(is_pow2(2));
        assert!(is_pow2(4));
        assert!(is_pow2(8));

        assert!(!is_pow2(6));
        assert!(!is_pow2(15));
        assert!(!is_pow2(63));
    }

    #[test]
    fn log2_works() {
        assert_eq!(log2(0), 0);
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(3), 1);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(16), 4);
        assert_eq!(log2(24), 4);
        assert_eq!(log2(32), 5);
    }

    #[test]
    fn div_round_works() {
        assert_eq!(div_ws_ceil(0), 0);
        assert_eq!(div_ws_ceil(1), 1);
        assert_eq!(div_ws_ceil(WORD_BIT_SIZE), 1);
        assert_eq!(div_ws_ceil(WORD_BIT_SIZE + 1), 2);
        assert_eq!(div_ws_ceil(WORD_BIT_SIZE * 2), 2);
        assert_eq!(div_ws_ceil(WORD_BIT_SIZE * 3), 3);
        assert_eq!(div_ws_ceil(WORD_BIT_SIZE * 3 + 1), 4);
    }

    #[test]
    fn weight_works() {
        assert!(weight(0b0000_0001 as usize) == 1);
        assert!(weight(0b0000_1101 as usize) == 3);
        assert!(weight(0b0000_0000 as usize) == 0);
        assert!(weight(0b1000_0000 as usize) == 1);
        assert!(weight(0b1111_1111 as usize) == 8);
        assert!(weight(Value::MAX as usize) == WORD_BIT_SIZE);
    }

    #[test]
    fn bin_comb_works() {
        // TODO: check that iter length = Ckn and all unrepeatable with given weight
        for comb in BinComb::new(5, 2) {
            println!("{comb:05b}");
        }
    }
}
