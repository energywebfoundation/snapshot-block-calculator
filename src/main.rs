use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    println!("{}", rng.gen_range(0..100));
}
