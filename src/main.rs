
use snapshot_block_calculator::calc_snapshot_block;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 19742311; // https://explorer.energyweb.org/block/19742311/transactions
  let end_block:u32 = 20427965; // https://explorer.energyweb.org/block/20427965/transactions
  let block_hash = "0xa63aee496c3e6ea55c02b64bed4683059c9476a3968dca06c00a06a8b35e3293"; // https://explorer.energyweb.org/block/20557319/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

