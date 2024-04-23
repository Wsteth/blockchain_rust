use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain { blocks: vec![] };
        let block = Block::new("genesis block", "0");
        blockchain.blocks.push(block);
        blockchain
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, &prev_block.hash);
        self.blocks.push(new_block);
    }
}
