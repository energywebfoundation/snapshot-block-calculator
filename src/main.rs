use rand::prelude::*;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
  let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
  let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

/// Returns a random number between the provided start_block and end_block numbers
/// The execution is reproducable given the same seed
/// The pseudo random number generator (rng) use is https://docs.rs/rand_pcg/latest/rand_pcg/struct.Lcg128Xsl64.html
/// https://rust-random.github.io/book/guide-seeding.html#a-string-or-any-hashable-data
fn calc_snapshot_block(start_block: u32, end_block: u32, block_hash: &str) -> u32 {
  let mut rng: Pcg64 = Seeder::from(block_hash).make_rng();
  rng.gen_range(start_block..end_block)
}
