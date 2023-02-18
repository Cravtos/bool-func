#[inline]
pub fn is_exp2(n: usize) -> bool {
    n != 0 && (n & (n - 1)) > 0
}