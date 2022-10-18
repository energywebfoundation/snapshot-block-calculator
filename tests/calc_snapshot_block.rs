
mod calc_snapshot_block {

    use snapshot_block_calculator::calc_snapshot_block;

    #[test]
    fn test_output_is_within_bounds() {
      let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
      let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
      let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 18127081);
      assert!(output > start_block);
      assert!(output < end_block);
    }

    #[test]
    fn test_output_is_reproducible() {
      let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
      let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
      let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
      let output_1 = calc_snapshot_block(start_block, end_block, block_hash);
      let output_2 = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output_1, output_2);
    }

    /// This test demonstrates that output is on the same proportional point on a range (given the same seed).
    /// This is demonstrated by using start&end sets which are the same size but an increment apart
    #[test]
    fn test_result_is_independant_of_incremented_range() {
      let start_block: u32 = 0;
      let end_block:u32 = 100;
      let range_increment: u32 = 100;
      let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
      let output_1 = calc_snapshot_block(start_block, end_block, block_hash);
      let output_2 = calc_snapshot_block(start_block+range_increment, end_block+range_increment, block_hash);
      assert_eq!(output_1+range_increment, output_2);
    }

    /// This test demonstrates that output is on the same proportional point on a range (given the same seed).
    /// This is demonstrated by using start&end sets which are proportional in size and showing that result is proportional
    #[test]
    fn test_result_is_independant_of_range_2() {
      let start_block: u32 = 0;
      let end_block:u32 = 100;
      let range_multiple: u32 = 2;
      let block_hash = "seed string 3"; 
      let output_1 = calc_snapshot_block(start_block, end_block, block_hash);
      let output_2 = calc_snapshot_block(start_block*range_multiple, end_block*range_multiple, block_hash);
      assert_eq!(output_1*range_multiple, output_2);
    }

    #[test]
    fn test_can_generate_different_outputs() {
      let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
      let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
      let block_hash_1 = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
      let block_hash_2 = "0x2e671cdbcdb49c5500f37e78f75ec2cf534f9b3522ca24c126e569a3424324f4"; // https://explorer.energyweb.org/block/18792404/transactions
      let output1= calc_snapshot_block(start_block, end_block, block_hash_1);
      let output2= calc_snapshot_block(start_block, end_block, block_hash_2);
      assert_ne!(output1, output2);
    }

    #[test]
    fn test_consortia_snapshot1() {
      let start_block: u32 = 17675146; // https://explorer.energyweb.org/block/17675146/transactions
      let end_block:u32 = 18197775; // https://explorer.energyweb.org/block/18197775/transactions
      let block_hash = "0x5a4cd6525687101f7143e27b961fc7ef999d10fc8937bcce9c9d2796bd3f19eb"; // https://explorer.energyweb.org/block/18812000/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 18059849); // https://explorer.energyweb.org/block/18059849/transactions
      assert!(output > start_block);
      assert!(output < end_block);
    }

    #[test]
    fn test_consortia_snapshot2() {
      let start_block: u32 = 18197776; // https://explorer.energyweb.org/block/17675146/transactions
      let end_block:u32 = 19142937; // https://explorer.energyweb.org/block/18197775/transactions
      let block_hash = "0x2d47301fe8af4a1d4d94f1868930eedf216ce995090ee8534c8fa5cbf2f291f3"; // https://explorer.energyweb.org/block/19153600/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 18648235); // https://explorer.energyweb.org/block/18059849/transactions
      assert!(output > start_block);
      assert!(output < end_block);
    }

    #[test]
    fn test_consortia_snapshot3() {
      let start_block: u32 = 19142938; // https://explorer.energyweb.org/block/19142938/transactions
      let end_block:u32 = 19742310; // https://explorer.energyweb.org/block/19742310/transactions
      let block_hash = "0xb424122b9379b1dff91dc5d15e2b25b0a55a90c4fe9c61e27f3181f6cc744ca3"; // https://explorer.energyweb.org/block/19857275/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 19190154); // https://explorer.energyweb.org/block/18059849/transactions
      assert!(output > start_block);
      assert!(output < end_block);
    }

    #[test]
    fn test_consortia_snapshot4() {
      let start_block: u32 = 19742311; // https://explorer.energyweb.org/block/19742311/transactions
      let end_block:u32 = 20427965; // https://explorer.energyweb.org/block/20427965/transactions
      let block_hash = "0xa63aee496c3e6ea55c02b64bed4683059c9476a3968dca06c00a06a8b35e3293"; // https://explorer.energyweb.org/block/20557319/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 20145052); // https://explorer.energyweb.org/block/20145052/transactions
      assert!(output > start_block);
      assert!(output < end_block);
    }

}