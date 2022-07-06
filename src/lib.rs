use rand::prelude::*;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;

/// Returns a random number between the provided start_block and end_block numbers
/// The execution is reproducable given the same seed
/// The pseudo random number generator (rng) use is https://docs.rs/rand_pcg/latest/rand_pcg/struct.Lcg128Xsl64.html
/// https://rust-random.github.io/book/guide-seeding.html#a-string-or-any-hashable-data
pub fn calc_snapshot_block(start_block: u32, end_block: u32, block_hash: &str) -> u32 {
  let mut rng: Pcg64 = Seeder::from(block_hash).make_rng();
  rng.gen_range(start_block..end_block)
}
