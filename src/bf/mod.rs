pub mod errors;
pub mod utils;

use errors::{BFError, Result};
use utils::*;

use rand::{distributions::Uniform, Rng};
use std::str::FromStr;

type Value = usize;

/// BF represents boolean function.
/// Arguments are stored in little-endian fashion.
#[derive(Debug)]
pub struct BF {
    /// Vector, holding function values for corresponding arguments.
    values: Vec<Value>,

    /// Amount of arguments boolean function takes.
    pub args_amount: usize,
}

impl BF {
    /// Creates boolean function which equals `0` for all arguments.
    ///
    /// # Errors
    /// Returns `BFError::NoArgs` if args_amount == 0
    pub fn zero(args_amount: usize) -> Result<Self> {
        if args_amount == 0 {
            return Err(BFError::NoArgs);
        }

        let cap = div_ws_ceil(pow2(args_amount));
        Ok(BF {
            values: vec![0; cap],
            args_amount,
        })
    }

    /// Creates boolean function which equals `1` for all arguments.
    ///
    /// # Errors
    /// Returns `BFError::NoArgs` if args_amount == 0
    pub fn one(args_amount: usize) -> Result<Self> {
        if args_amount == 0 {
            return Err(BFError::NoArgs);
        }

        let cap = div_ws_ceil(pow2(args_amount));
        let bits_in_last_factor = mod_ws(pow2(args_amount));
        let mut values = vec![Value::MAX; cap];

        // Set unused bits to zero;
        if bits_in_last_factor != 0 {
            values[cap - 1] &= (1 << bits_in_last_factor) - 1;
        }

        Ok(BF {
            values,
            args_amount,
        })
    }

    /// Creates boolean function which has random result for all arguments.
    /// Result is uniformly distributed.
    ///
    /// # Errors
    /// Returns `BFError::NoArgs` if args_amount == 0
    pub fn random(args_amount: usize) -> Result<Self> {
        if args_amount == 0 {
            return Err(BFError::NoArgs);
        }

        let cap = div_ws_ceil(pow2(args_amount));
        let bits_in_last_factor = mod_ws(pow2(args_amount));

        let rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(Value::MIN, Value::MAX);
        let mut values: Vec<Value> = rng.sample_iter(uniform).take(cap).collect();

        // Set unused bits to zero;
        if bits_in_last_factor != 0 {
            values[cap - 1] &= (1 << bits_in_last_factor) - 1;
        }

        Ok(BF {
            values,
            args_amount,
        })
    }

    /// Calculates weight of function
    pub fn weight(&self) -> usize {
        // NOTE: function assumes that unused bits in value set to zero.

        self.values
            .iter()
            .fold(0, |acc, &factor| acc + weight(factor))
    }
}

// TODO: implement
impl FromStr for BF {
    type Err = BFError;

    fn from_str(s: &str) -> Result<Self> {
        todo!();
    }
}

impl ToString for BF {
    fn to_string(&self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_works() {
        let args_amount = 4;
        let bf = BF::zero(args_amount).expect("args_amount is not zero");
        for value in &bf.values {
            assert!(*value == 0);
        }

        // expected_length holds ceil((2^args_amount)/WORD_BIT_SIZE)
        let expected_length = (pow2(args_amount) + WORD_BIT_SIZE - 1) / WORD_BIT_SIZE;
        assert!(bf.values.len() == expected_length);
        assert!(bf.args_amount == args_amount);
    }

    #[test]
    fn one_works() {
        let args_amount = WORD_SIZE;
        let bf = BF::one(args_amount).expect("args_amount is not zero");

        for value in &bf.values[..bf.values.len() - 1] {
            assert!(*value == Value::MAX);
        }

        // expected_length holds ceil((2^args_amount)/WORD_BIT_SIZE)
        let expected_length = (pow2(args_amount) + WORD_BIT_SIZE - 1) / WORD_BIT_SIZE;
        assert!(bf.values.len() == expected_length);
        assert!(bf.args_amount == args_amount);

        assert!(bf.weight() == pow2(args_amount))
    }

    #[test]
    fn weight_works() {
        let args_amount = 2;
        let bf = BF::one(args_amount).expect("args_amount is not zero");
        assert!(bf.weight() == 4);

        let args_amount = log2(WORD_BIT_SIZE);
        let bf = BF::one(args_amount).expect("args_amount is not zero");
        assert!(bf.weight() == WORD_BIT_SIZE);
    }
}
