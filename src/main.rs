use lib::Blockchain;
use sled::Result;

fn main() -> Result<()> {
    let mut blockchain = Blockchain::new()?;
    blockchain.add_block("test 3")?;
    blockchain.add_block("test 4")?;

    for block in blockchain.iter() {
        println!("{} {} {}", block.hash, block.data, block.prev_block_hash);
    }
    Ok(())
}
