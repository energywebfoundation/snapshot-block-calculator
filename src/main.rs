
use snapshot_block_calculator::calc_snapshot_block;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 18197776; // https://explorer.energyweb.org/block/17675146/transactions
  let end_block:u32 = 19142937; // https://explorer.energyweb.org/block/18197775/transactions
  let block_hash = "0x2d47301fe8af4a1d4d94f1868930eedf216ce995090ee8534c8fa5cbf2f291f3"; // https://explorer.energyweb.org/block/19153600/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

