use crate::utils;

/// TODO: починить докстринги
/// TODO: сделать генерик размер фактора
pub struct BF {
    values: Vec<u64>,
    args_amount: usize,
}

impl BF {
    pub fn zero(args_amount: usize) -> Self {
        let cap = 1;
        BF {
            values: vec![0; cap],
            args_amount
        }
    }

    pub fn one(args_amount: usize) -> Self {
        let cap = 1;
        BF {
            values: vec![1; cap],
            args_amount
        }
    }
}

// 1 << args_amount 
// |2^args_amount / size_in_bits_of(T)|