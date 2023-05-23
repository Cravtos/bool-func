pub mod errors;

use crate::bf::utils::{div_ws, div_ws_ceil, mod_ws, WORD_BIT_SIZE};
use errors::{BMError, Result};
use rand::{distributions::Uniform, Rng};
use std::{fmt, str::FromStr};

use crate::Value;

#[derive(Debug)]
pub struct BM {
    mat: Vec<Value>,
    rows: usize,
    cols: usize,
}

impl BM {
    pub fn zero(rows: usize, cols: usize) -> Result<Self> {
        if cols == 0 || rows == 0 {
            return Err(BMError::ZeroDim(rows, cols));
        }

        let cap = div_ws_ceil(rows * cols);
        let mat = vec![0; cap];

        Ok(BM { mat, rows, cols })
    }

    pub fn random(rows: usize, cols: usize) -> Result<Self> {
        if cols == 0 || rows == 0 {
            return Err(BMError::ZeroDim(rows, cols));
        }

        let cap = div_ws_ceil(rows * cols);
        let bits_in_last_factor = mod_ws(rows * cols);

        let rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(Value::MIN, Value::MAX);
        let mut mat: Vec<Value> = rng.sample_iter(uniform).take(cap).collect();

        if bits_in_last_factor != 0 {
            mat[cap - 1] &= (1 << bits_in_last_factor) - 1;
        }

        Ok(BM { mat, rows, cols })
    }

    pub fn rank(&self) -> usize {
        unimplemented!()
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        let factor = div_ws(row * self.cols + col);
        let bit = mod_ws(row * self.cols + col);
        ((self.mat[factor] >> bit) & 1) as u8
    }

    pub fn set(&mut self, row: usize, col: usize) {
        let factor = div_ws(row * self.cols + col);
        let bit = mod_ws(row * self.cols + col);

        let mask = 1 << bit;
        self.mat[factor] |= mask;
    }

    pub fn unset(&mut self, row: usize, col: usize) {
        let factor = div_ws(row * self.cols + col);
        let bit = mod_ws(row * self.cols + col);

        let mask = 1 << bit;
        let mask = !mask;
        self.mat[factor] &= mask;
    }

    pub fn gaussian_elimination(&mut self) {
        let mut cur_row = 0;
        let mut cur_col = 0;

        while cur_row < self.rows && cur_col < self.cols {
            // find row with 1 in col
            let mut pivot = cur_row;
            for row in cur_row..self.rows {
                if self.get(row, cur_col) == 1 {
                    pivot = row;
                    break;
                }
            }

            // if no pivot, go to next column
            if self.get(pivot, cur_col) == 0 {
                cur_col += 1;
                continue;
            }

            // swap row and pivot
            for col in 0..self.cols {
                let to_pivot = self.get(cur_row, col);
                let to_row = self.get(pivot, col);

                match to_pivot {
                    0 => self.unset(pivot, col),
                    1 => self.set(pivot, col),
                    _ => unreachable!(),
                }

                match to_row {
                    0 => self.unset(cur_row, col),
                    1 => self.set(cur_row, col),
                    _ => unreachable!(),
                }
            }

            // xor all elements below row by row
            for row in (cur_row + 1)..self.rows {
                if self.get(row, cur_col) == 0 {
                    continue;
                }

                for col in 0..self.cols {
                    let a = self.get(cur_row, col);
                    let b = self.get(row, col);

                    let r = a ^ b;
                    match r {
                        0 => self.unset(row, col),
                        1 => self.set(row, col),
                        _ => unreachable!(),
                    }
                }
            }

            cur_row += 1;
            cur_col += 1;
        }
    }
}

impl fmt::Display for BM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let bit = self.get(row, col).to_string();
                string.push_str(&bit);
            }

            if row != self.rows - 1 {
                string.push('\n');
            }
        }
        write!(f, "{}", string)
    }
}

impl FromStr for BM {
    type Err = BMError;

    // Converts string like "1101\n1111\n0000" to boolean matrix
    fn from_str(s: &str) -> Result<Self> {
        let str_rows: Vec<&str> = s.split("\n").collect();

        let rows = str_rows.len();
        let cols = str_rows[0].len();

        if rows == 0 || cols == 0 {
            return Err(BMError::ZeroDim(rows, cols));
        }

        if rows > 1 {
            let consistent = str_rows.windows(2).all(|w| w[0].len() == w[1].len());
            if !consistent {
                return Err(BMError::InconsistentDim);
            }
        }

        let mut bm = BM::zero(rows, cols).unwrap();
        for row in 0..rows {
            for col in 0..cols {
                let bit = str_rows[row].chars().nth(col).unwrap();
                match bit {
                    '1' => bm.set(row, col),
                    '0' => (),
                    _ => return Err(BMError::InvalidStr(bit)),
                }
            }
        }

        Ok(bm)
    }
}

mod tests {
    use super::*;

    #[test]
    fn random_works() {
        let bm = BM::random(3, 8).unwrap();
        println!("{bm}");
    }

    #[test]
    fn from_str_works() {
        let s = "0110\n1101\n1111";
        let bm = BM::from_str(s).unwrap();
        assert_eq!(bm.to_string(), s);
    }

    #[test]
    fn gauss_works() {
        let s = "0110\n1101\n1111\n1111";
        let mut bm = BM::from_str(s).unwrap();
        bm.gaussian_elimination();
        println!("{bm}");
    }
}
