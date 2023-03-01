pub mod errors;
pub mod utils;

use errors::{BFError, Result};
use std::fmt;
use utils::*;

use rand::{distributions::Uniform, Rng};
use std::str::FromStr;

type Value = u128;

/// BF represents boolean function.
/// Arguments are stored in little-endian fashion.
#[derive(Debug, PartialEq, Eq)]
pub struct BF {
    /// Vector, holding function values for corresponding arguments.
    ///
    /// Least significant bits are in `values[0]`.
    /// First bit of `value[0]` is the least significant bit.
    values: Vec<Value>,

    /// Amount of arguments boolean function takes.
    /// Can't be zero.
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

impl FromStr for BF {
    type Err = BFError;

    /// Converts string to boolean function
    ///
    /// # Errors
    /// Returns `BFError::InvalidString` if `s` doesn't consist of zeros and ones,
    /// or `BFError::NotPowTwo` if `len(s)` is not a power of 2.
    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        if len == 1 || !is_pow2(len) {
            return Err(BFError::NotPowTwo(len));
        }

        let cap = div_ws_ceil(len);
        let mut values: Vec<Value> = vec![0; cap];

        for (i, bit) in s.chars().rev().enumerate() {
            let bit = match bit {
                '0' => 0,
                '1' => 1,
                _ => return Err(BFError::InvalidString(s.to_string())),
            };

            let v_idx = div_ws(i);

            values[v_idx] = (values[v_idx] << 1) | bit;
        }

        Ok(BF {
            values,
            args_amount: log2(len),
        })
    }
}

impl fmt::Display for BF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = self
            .values
            .iter()
            .rev()
            .map(|value| {
                let mut s = String::with_capacity(WORD_BIT_SIZE); // NOTE: allocations can be reduced to one.
                for i in 0..WORD_BIT_SIZE {
                    let bit = (value >> i) & 1;
                    let char = match bit {
                        0 => '0',
                        1 => '1',
                        _ => unreachable!(),
                    };
                    s.push(char);
                }
                s
            })
            .collect();

        let bits_in_last_factor = mod_ws(pow2(self.args_amount));
        if bits_in_last_factor != 0 {
            let used_bits = string.len() - WORD_BIT_SIZE + bits_in_last_factor;
            string = string[..used_bits].to_owned(); // NOTE: here could be unneeded allocation
        }
        write!(f, "{}", string)
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

    #[test]
    fn str_works() {
        fn test_valid(s: &str) {
            let str_before = String::from(s);
            let bf_before = str_before.parse::<BF>().expect("Can parse string");
            let str_after = bf_before.to_string();
            let bf_after = str_after.parse::<BF>().expect("Can parse string");
            assert!(str_before == str_after);
            assert!(bf_before == bf_after);
        }

        test_valid("1111");
        test_valid("00");
        test_valid("11111111");
        test_valid("1011011101110101");

        fn test_not_boolen(s: &str) {
            let res = s.parse::<BF>();
            match res {
                Ok(_) => panic!("Should return error"),
                Err(err) => assert_eq!(err, BFError::InvalidString(s.to_string())),
            }
        }

        test_not_boolen("20");
        test_not_boolen("3333");
        test_not_boolen("111s");

        fn test_not_pow_two(s: &str) {
            let res = s.parse::<BF>();
            match res {
                Ok(_) => panic!("Should return error"),
                Err(err) => assert_eq!(err, BFError::NotPowTwo(s.len())),
            }
        }

        test_not_pow_two("0");
        test_not_pow_two("111");
        test_not_pow_two("11111111111111111111111111111");
    }
}
