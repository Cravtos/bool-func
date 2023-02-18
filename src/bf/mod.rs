mod utils;
use utils::*;

type Value = usize;

pub const WORD_SIZE: Value = std::mem::size_of::<Value>() as Value;
pub const WORD_BIT_SIZE: Value = WORD_SIZE * 8;

/// BF represents boolean function.
/// Arguments are stored in little-endian fashion.
pub struct BF {
    /// Vector, holding function values for corresponding arguments.
    values: Vec<Value>,

    /// Amount of arguments boolean function takes.
    pub args_amount: usize,
}

impl BF {
    /// Creates boolean function which equals `0` for all arguments.
    pub fn zero(args_amount: usize) -> Self {
        let cap = div_round(pow2(args_amount));
        BF {
            values: vec![0; cap],
            args_amount,
        }
    }

    /// Creates boolean function which equals `1` for all arguments.
    pub fn one(args_amount: usize) -> Self {
        let cap = div_round(pow2(args_amount));
        BF {
            values: vec![Value::MAX; cap],
            args_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_works() {
        let args_amount = 4;
        let bf = BF::zero(args_amount);
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
        let bf = BF::one(args_amount);
        for value in &bf.values {
            assert!(*value == Value::MAX);
        }

        // expected_length holds ceil((2^args_amount)/WORD_BIT_SIZE)
        let expected_length = (pow2(args_amount) + WORD_BIT_SIZE - 1) / WORD_BIT_SIZE;
        assert!(bf.values.len() == expected_length);
        assert!(bf.args_amount == args_amount);
    }
}
