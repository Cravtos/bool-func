use bool_func::bf::BF;
use bool_func::bf::utils::pow2;

fn main() {
    // TODO: move to tests
    for _ in 0..30 {
        let args_amount = 16;
        let bf = BF::random(args_amount);
        println!("weight/bits = {:.3}", bf.weight() as f64 / pow2(args_amount) as f64);
    }
}
