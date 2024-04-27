use sled::Result;

use crate::Block;

pub struct Blockchain {
    tip: String,
    db: sled::Db,
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Blockchain::new().unwrap()
    }
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        let db = sled::open("db/blocks")?;
        let tip = match db.get("l")? {
            Some(last) => String::from_utf8(last.to_vec()).unwrap(),
            None => {
                let genesis = Block::new("genesis block", "0");
                db.insert("l", genesis.hash.as_bytes())?;
                db.insert(genesis.hash.clone(), genesis.serialize().as_bytes())?;
                genesis.hash
            }
        };
        db.flush()?;
        Ok(Blockchain { tip, db })
    }

    pub fn add_block(&mut self, data: &str) -> Result<Block> {
        let last_serialized_block =
            String::from_utf8(self.db.get(&self.tip)?.unwrap().to_vec()).unwrap();
        let last_block = Block::deserialize(&last_serialized_block);
        println!("last hash: {}", last_block.hash);

        let new_block = Block::new(data, &last_block.hash);
        self.db.insert("l", new_block.hash.as_bytes())?;
        self.db
            .insert(new_block.hash.clone(), new_block.serialize().as_bytes())?;
        self.tip.clone_from(&new_block.hash);
        self.db.flush()?;
        Ok(new_block)
    }
    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            block_chain: self,
        }
    }
}

pub struct BlockchainIterator<'bc> {
    current_hash: String,
    block_chain: &'bc Blockchain,
}

impl<'bc> Iterator for BlockchainIterator<'bc> {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        let db = &self.block_chain.db;
        if let Ok(encoded_block) = db.get(&self.current_hash) {
            return match encoded_block {
                Some(serialized_block) => {
                    let block =
                        Block::deserialize(&String::from_utf8(serialized_block.to_vec()).unwrap());
                    self.current_hash.clone_from(&block.prev_block_hash);
                    Some(block)
                }
                None => None,
            };
        }
        None
    }
}
