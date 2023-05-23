pub mod bf;
pub mod bm;

#[cfg(test)]
type Value = u8;
#[cfg(not(test))]
type Value = u128;
