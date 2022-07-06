use rand::prelude::*;
use rand_seeder::{SipHasher};
use rand_chacha::{ChaCha20Rng};

/// Returns a random number between the provided start_block and end_block numbers
/// The execution is reproducable given the same seed.
/// First a SipHasher is used to generate a seed from a blockhash as a string https://rust-random.github.io/book/guide-seeding.html#a-string-or-any-hashable-data
/// Then, a cryptopgraphically secure RNG (ChaCha) is used to generate the block number
pub fn calc_snapshot_block(start_block: u32, end_block: u32, block_hash: &str) -> u32 {
  // First we create a SipRng:
  let hasher = SipHasher::from(block_hash);
  let mut hasher_rng = hasher.into_rng();

  // Now, we use hasher_rng to create a seed:
  let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
  hasher_rng.fill(&mut seed);

  // And create the RNG that seed
  let mut rng = ChaCha20Rng::from_seed(seed);

  // Generate snapshot block from 
  rng.gen_range(start_block..end_block)
}
