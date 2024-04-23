use lib::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("test 1");
    blockchain.add_block("test 2");

    for block in &blockchain.blocks {
        println!("{} {} {}", block.hash, block.data, block.prev_block_hash);
    }
}
