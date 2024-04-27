use chrono::Local;
use num::{BigUint, One};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const TARGET_BITS: u32 = 16;
const MAX_NONCE: u64 = u64::MAX;
#[derive(Serialize, Deserialize)]
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
        let mut block = Block {
            timestamp,
            hash: String::new(),
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            nonce: 0,
        };
        block.proof_of_work();
        block
    }

    fn proof_of_work(&mut self) {
        let mut target: BigUint = BigUint::one();
        target <<= 256 - TARGET_BITS;
        let mut hasher = Sha256::new();
        let mut nonce = 0u64;
        let start = Local::now().timestamp_millis();
        println!("Mining the block containing \"{}\"", self.data);
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hasher = Sha256::new();
            hasher.update(&data);

            if target > BigUint::from_bytes_be(&hasher.clone().finalize()) {
                let end = Local::now().timestamp_millis();
                println!("Hash: {:x}", hasher.clone().finalize());
                println!("Nonce: {} Mining Duration: {}ms", nonce, end - start);
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n\n");
        self.hash = format!("{:x}", hasher.finalize());
        self.nonce = nonce;
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let data = format!(
            "{}{}{}{}{}",
            self.prev_block_hash, self.data, self.timestamp, TARGET_BITS, nonce
        );
        data.as_bytes().to_vec()
    }
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn deserialize(data: &str) -> Block {
        serde_json::from_str(data).unwrap()
    }
}
