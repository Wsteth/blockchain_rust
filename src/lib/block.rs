use chrono::Local;

use crate::proof_of_work::ProofOfWork;

pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Block {
        let timestamp = Local::now().timestamp();
        let block = Block {
            timestamp,
            hash: String::new(),
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            nonce: 0,
        };
        let mut pow = ProofOfWork::new(block);
        let (nonce, hash) = pow.run();
        Block {
            timestamp,
            hash,
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            nonce,
        }
    }
}
