pub mod errors;
pub mod utils;

use errors::{BFError, Result};
use std::fmt;
use utils::*;

use itertools::Itertools;
use rand::{distributions::Uniform, Rng};
use std::str::FromStr;

#[cfg(test)]
type Value = u8;
#[cfg(not(test))]
type Value = u128;

/// BF represents boolean function.
/// Arguments are stored in little-endian fashion.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BF {
    /// Vector, holding function values for corresponding arguments.
    ///
    /// Least significant bits are in `values[0]`.
    /// First bit of `value[0]` is the least significant bit.
    pub values: Vec<Value>,

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
            values[0] &= (1 << bits_in_last_factor) - 1;
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
            values[0] &= (1 << bits_in_last_factor) - 1;
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

    /// Calculates Mobuis transform inplace.
    pub fn mobius(&mut self) -> &mut Self {
        let m = log2(WORD_BIT_SIZE);
        for value in self.values.iter_mut() {
            for i in 0..m {
                *value ^= (*value << pow2(i)) & utils::halving_mask(i);
            }
        }

        // zero out leading trash if args_amount < log2(WORD_BIT_SIZE)
        if self.args_amount < m {
            let bits_in_last_factor = mod_ws(pow2(self.args_amount));
            self.values[0] &= (1 << bits_in_last_factor) - 1;
            return self;
        }

        for i in 0..self.args_amount - m {
            let cs = pow2(i);
            for j in (0..self.values.len() / cs).step_by(2) {
                for k in 0..cs {
                    self.values[(j + 1) * cs + k] ^= self.values[j * cs + k];
                }
            }
        }

        self
    }

    // Evaluates boolean function on given argument
    pub fn eval(&self, args: usize) -> u8 {
        let factor = div_ws(args);
        let bit_in_factor = mod_ws(args);
        ((self.values[factor] >> bit_in_factor) & 1) as u8
    }

    // Change function to evaluate to one on given argument
    pub fn set(&mut self, args: usize) -> Result<()> {
        if args >= pow2(self.args_amount) {
            Err(BFError::ArgOutOfBounds {
                given: args,
                bounds: pow2(self.args_amount),
            })?;
        }

        let factor = div_ws(args);
        let bit_in_factor = mod_ws(args);
        let mask = 1 << bit_in_factor;
        self.values[factor] |= mask;

        Ok(())
    }

    // Change function to evaluate to zero on given argument
    pub fn unset(&mut self, args: usize) -> Result<()> {
        if args >= pow2(self.args_amount) {
            Err(BFError::ArgOutOfBounds {
                given: args,
                bounds: pow2(self.args_amount),
            })?;
        }

        let factor = div_ws(args);
        let bit_in_factor = mod_ws(args);
        let mask = 1 << bit_in_factor;
        let mask = !mask;
        self.values[factor] &= mask;

        Ok(())
    }

    // Get arithmetic normal form of function
    pub fn anf(&self) -> String {
        let mut bf_copy = self.clone();
        let bf_mob = bf_copy.mobius();

        if bf_mob.weight() == 0 {
            return String::from("0");
        }

        let mut anf: String = (1..pow2(bf_mob.args_amount) as u128)
            .into_iter()
            .filter(|&args| bf_mob.eval(args as usize) == 1)
            .map(|args| {
                (0..WORD_BIT_SIZE)
                    .into_iter()
                    .filter(|&i| (args >> i) & 1 == 1)
                    .map(|i| format!("x{}", bf_mob.args_amount - i))
                    .intersperse(String::from("&"))
                    .collect::<String>()
            })
            .intersperse(String::from(" + "))
            .collect();

        if bf_mob.eval(0) == 1 {
            let mut one = String::from("1");
            if !anf.is_empty() {
                one.push_str(" + ");
            }
            anf = one + &anf;
        }

        anf
    }

    // Calculate function degree
    pub fn deg(&self) -> usize {
        let mut bf_copy = self.clone();
        let bf_mob = bf_copy.mobius();

        let n = pow2(self.args_amount);
        if bf_mob.eval(n - 1) == 1 {
            return self.args_amount;
        }

        let mut deg = 0;
        for arg in (0..n - 1).rev() {
            if bf_mob.eval(arg) == 0 {
                continue;
            }

            let weight = utils::weight(arg as Value);
            if weight > deg {
                deg = weight;
            }
        }

        deg
    }

    // Get walsh adamar coefficients
    pub fn walsh_adamar(&self) -> Vec<i32> {
        let mut char_vec = (0..pow2(self.args_amount))
            .into_iter()
            .map(|arg| match self.eval(arg) {
                0 => 1,
                1 => -1,
                _ => panic!("function evaluated to non binary"),
            })
            .collect::<Vec<i32>>();

        for i in 0..self.args_amount {
            let cs = pow2(i);
            for j in 0..char_vec.len() / cs {
                if j & 1 == 0 {
                    // is even
                    for k in 0..cs {
                        char_vec[j * cs + k] += char_vec[(j + 1) * cs + k]; // a + b
                    }
                } else {
                    // is odd
                    for k in 0..cs {
                        char_vec[j * cs + k] =
                            char_vec[(j - 1) * cs + k] - 2 * char_vec[j * cs + k];
                        // a + b - 2b = a - b
                    }
                }
            }
        }

        char_vec
    }

    // Calculate maximal correlation immunity of a function.
    pub fn cor(&self) -> usize {
        let wac = self.walsh_adamar();

        for k in 1..=self.args_amount {
            for comb in BinComb::new(self.args_amount, k) {
                if wac[comb] != 0 {
                    return k - 1;
                }
            }
        }

        self.args_amount
    }
}

impl FromStr for BF {
    type Err = BFError;

    /// Converts string to boolean function
    /// First char in string correspond to arguments with values zero.
    /// Last char in string to arguments with values one.
    ///
    /// # Errors
    /// Returns `BFError::InvalidString` if `s` doesn't consist of zeros and ones,
    /// or `BFError::NotPowTwo` if `len(s)` is not a power of 2.
    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        if len == 1 || !is_pow2(len) {
            return Err(BFError::NotPowTwo(len));
        }

        let mut bf = BF::zero(log2(len)).expect("length not zero");

        for (i, bit) in s.chars().enumerate() {
            match bit {
                '0' => (), // value already zero
                '1' => bf.set(i)?,
                _ => return Err(BFError::InvalidString(s.to_string())),
            };
        }

        Ok(bf)
    }
}

impl fmt::Display for BF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: String = (0..pow2(self.args_amount))
            .map(|arg| self.eval(arg).to_string())
            .collect();

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
        test_valid("0001");
        test_valid("10111101");
        test_valid("1011011101110101");
        test_valid("10000000000000000000001000000000");

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

    #[test]
    fn eval_works() {
        let bf = BF::from_str("1010110011110000").expect("Can convert");
        // TODO: iterate over string
        assert_eq!(bf.eval(0), 1);
        assert_eq!(bf.eval(1), 0);
        assert_eq!(bf.eval(2), 1);
        assert_eq!(bf.eval(8), 1);
        assert_eq!(bf.eval(9), 1);
        assert_eq!(bf.eval(14), 0);
        assert_eq!(bf.eval(15), 0);
    }

    #[test]
    fn set_works() {
        fn test_set_unset(bf: &mut BF, args: usize) {
            assert_eq!(bf.eval(args), 0);
            bf.set(args).expect("Valid arg");
            assert_eq!(bf.eval(args), 1);
            bf.unset(args).expect("Valid arg");
            assert_eq!(bf.eval(args), 0);
        }

        let mut bf = BF::zero(log2(WORD_BIT_SIZE) + 1).expect("Args amount not zero");
        for i in 0..pow2(bf.args_amount) {
            test_set_unset(&mut bf, i);
        }
    }

    #[test]
    fn mobius_random_reversability() {
        for i in 0..100 {
            let mut bf = BF::random(i % 16 + 1).expect("arg amount is not zero");
            let old = bf.clone();
            bf.mobius();
            bf.mobius();
            println!("{}", i % 16 + 1);
            assert!(bf == old);
        }
    }

    // #[test]
    // fn mobius_31_factor_reversability() {
    //     let mut bf = BF::random(31).expect("arg amount is not zero");
    //     let old = bf.clone();
    //     bf.mobius();
    //     bf.mobius();
    //     assert!(bf == old);
    // }

    #[test]
    fn mobius_transform_const0_anf() {
        let mut bf = BF::zero(16).unwrap();
        let anf = bf.anf();

        bf.mobius();
        assert_eq!(bf.to_string(), "0".repeat(pow2(16) as usize));
        assert_eq!(anf, "0");
    }

    #[test]
    fn mobius_transform_const1_anf() {
        let mut bf = BF::one(16).unwrap();
        let anf = bf.anf();

        bf.mobius();
        assert_eq!(bf.to_string(), "1".to_owned() + &"0".repeat(pow2(16) - 1));
        assert_eq!(anf, "1");
    }

    #[test]
    fn anf_works() {
        let bf = BF::from_str("0001000100011110000100010001111000010001000111101110111011100001")
            .expect("can convert");
        assert_eq!(bf.anf(), "x6&x5 + x4&x3 + x2&x1");
        assert_eq!(bf.deg(), 2);

        let mut bf = BF::from_str("11000110").expect("can convert");
        bf.mobius();
        assert_eq!(bf.anf(), "1 + x3 + x3&x1 + x2&x1");

        let mut bf = BF::from_str("1111").expect("can convert");
        bf.mobius();
        assert_eq!(bf.anf(), "1 + x2 + x1 + x2&x1");

        let mut bf = BF::from_str("0000").expect("can convert");
        bf.mobius();
        assert_eq!(bf.anf(), "0");
    }

    #[test]
    fn degree_works() {
        let bf = BF::one(16).unwrap();
        assert_eq!(bf.deg(), 0);

        let bf = BF::zero(16).unwrap();
        assert_eq!(bf.deg(), 0);

        let bf = BF::from_str("0001").unwrap();
        assert_eq!(bf.deg(), 2);

        let bf = BF::from_str("00000001").unwrap();
        assert_eq!(bf.deg(), 3);

        let bf = "1".to_owned() + &"0".repeat(pow2(16) - 1);
        let bf = BF::from_str(&bf).unwrap();
        assert_eq!(bf.deg(), 16);
    }

    #[test]
    fn walsh_adamar_works() {
        let bf = BF::from_str("0110").unwrap();
        let wac = bf.walsh_adamar();
        assert_eq!(wac, vec![0, 0, 0, 4]);

        let bf = BF::from_str("0001000100011110").unwrap();
        let wac = bf.walsh_adamar();
        println!("{wac:?}");
        // assert_eq!(wac, vec![0, 0, 0, 4]);

        for i in 1..=3 {
            let bf = BF::one(i * 3).unwrap();
            let wac = bf.walsh_adamar();
            let mut expected = vec![0i32; pow2(i * 3)];
            expected[0] = -(pow2(i * 3) as i32);
            assert_eq!(wac, expected);
        }
    }

    #[test]
    fn cor_works() {
        let args_amount = 28;

        let bf = BF::one(args_amount).unwrap();
        assert_eq!(bf.cor(), args_amount);

        let bf = BF::zero(args_amount).unwrap();
        assert_eq!(bf.cor(), args_amount);

        let bf = BF::from_str("01101001").unwrap();
        assert_eq!(bf.cor(), 2);
    }
}
