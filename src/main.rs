use std::time::Instant;

use bool_func::bf::utils::{log2, pow2};
use bool_func::bf::BF;

fn check_weight() {
    for i in 2..=31 {
        let args_amount = i;

        let bf = match BF::random(args_amount) {
            Ok(bf) => bf,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!(
            "args = {i:2}; weight/bits = {:.3}",
            bf.weight() as f64 / pow2(args_amount) as f64
        );
    }
}

fn measure_walsh() {
    let bf = BF::random(32).unwrap();

    let start = Instant::now();
    let wac = bf.walsh_adamar();
    let duration = start.elapsed();

    println!("Time taken: {} seconds", duration.as_secs());
    println!("Args amount: {}", log2(wac.len()));
}

fn measure_cor() {
    let bf = BF::one(28).unwrap();

    let start = Instant::now();
    let cor = bf.cor();
    let duration = start.elapsed();

    println!("Time taken: {} seconds", duration.as_secs());
    println!("Cor immunity: {}", cor);
}

fn find_avc_limit() -> usize {
    const N: usize = 32;
    for i in 1..=N {
        println!("Calculating autocor for {i}...");
        let bf = BF::one(i).unwrap();
        let acv = bf.autocor();
        if !acv.iter().all(|v| *v == 1 << i) {
            println!("Wrong for {i}!");
            return i - 1;
        }
    }

    N
}

fn main() {
    let n = find_avc_limit();
    println!("Autocor limit: {n}");
    measure_cor();
    measure_walsh();
    check_weight();
}
