
use snapshot_block_calculator::calc_snapshot_block;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
  let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
  let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

