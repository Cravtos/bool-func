use bool_func::bf::utils::pow2;
use bool_func::bf::BF;

fn main() {
    for i in 2..34 {
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
