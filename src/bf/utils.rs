use super::{Value, WORD_BIT_SIZE, WORD_SIZE};

#[inline]
pub fn is_pow2(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[inline]
pub fn pow2(n: Value) -> Value {
    1 << n
}

/// Returns floor(log2(n))
#[inline]
pub fn log2(mut n: Value) -> Value {
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

#[inline]
pub fn div_round(n: Value) -> Value {
    (n + (WORD_BIT_SIZE - 1)) >> log2(WORD_BIT_SIZE)
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
        assert_eq!(div_round(0), 0);
        assert_eq!(div_round(1), 1);
        assert_eq!(div_round(WORD_BIT_SIZE), 1);
        assert_eq!(div_round(WORD_BIT_SIZE + 1), 2);
        assert_eq!(div_round(WORD_BIT_SIZE * 2), 2);
        assert_eq!(div_round(WORD_BIT_SIZE * 3), 3);
        assert_eq!(div_round(WORD_BIT_SIZE * 3 + 1), 4);
    }
}
