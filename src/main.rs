
use snapshot_block_calculator::calc_snapshot_block;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 19142938; // https://explorer.energyweb.org/block/19142938/transactions
  let end_block:u32 = 19742310; // https://explorer.energyweb.org/block/19742310/transactions
  let block_hash = "0xb424122b9379b1dff91dc5d15e2b25b0a55a90c4fe9c61e27f3181f6cc744ca3"; // https://explorer.energyweb.org/block/19857275/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

