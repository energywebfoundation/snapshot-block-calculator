
use snapshot_block_calculator::calc_snapshot_block;

/// Executes calc_snapshot_block.
/// Change values as necessary.
fn main() {
  let start_block: u32 = 17675146; // https://explorer.energyweb.org/block/17675146/transactions
  let end_block:u32 = 18197775; // https://explorer.energyweb.org/block/18197775/transactions
  let block_hash = "0x5a4cd6525687101f7143e27b961fc7ef999d10fc8937bcce9c9d2796bd3f19eb"; // https://explorer.energyweb.org/block/18812000/transactions
  println!("{}", calc_snapshot_block(start_block, end_block, block_hash));
}

