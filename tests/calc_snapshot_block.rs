
mod calc_snapshot_block {

    use snapshot_block_calculator::calc_snapshot_block;

    #[test]
    fn test_output_is_within_bounds() {
      let start_block: u32 = 17690000; // https://explorer.energyweb.org/block/17690000/transactions
      let end_block:u32 = 18200000; // https://explorer.energyweb.org/block/18200000/transactions
      let block_hash = "0xfdd6d56dc922bf093cd69abb72f4b1d33d1a4a9cd7978a04c59f97ad0345bada"; // https://explorer.energyweb.org/block/18778013/transactions
      let output = calc_snapshot_block(start_block, end_block, block_hash);
      assert_eq!(output, 18039370);
      assert!(output > start_block);
      assert!(output < end_block);
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
}