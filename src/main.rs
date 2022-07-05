use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    let start_block = 17690000;
    let end_block = 18200000;
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    println!("{}", rng.gen_range(start_block..end_block));
}
