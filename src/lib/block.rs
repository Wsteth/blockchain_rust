use chrono::Local;
use sha2::{Digest, Sha256};

pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub hash: String,
    pub prev_block_hash: String,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Block {
        let timestamp = Local::now().timestamp();
        let hash = generate_hash(prev_block_hash, timestamp, data);
        Block {
            timestamp,
            data: data.to_string(),
            hash,
            prev_block_hash: prev_block_hash.to_string(),
        }
    }
}

fn generate_hash(prev_block_hash: &str, timestamp: i64, data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", prev_block_hash, timestamp, data));
    format!("{:x}", hasher.finalize())
}
