use chrono::Local;
use num::{BigUint, One};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

const TARGET_BITS: u32 = 16;
const MAX_NONCE: u64 = u64::MAX;
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: &str) -> Block {
        let timestamp = Local::now().timestamp();
        let mut block = Block {
            timestamp,
            hash: String::new(),
            transactions,
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
            "{}{:?}{}{}{}",
            self.prev_block_hash,
            self.get_hash_transactions(),
            self.timestamp,
            TARGET_BITS,
            nonce
        );
        data.as_bytes().to_vec()
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(data: &str) -> Block {
        serde_json::from_str(data).unwrap()
    }

    fn get_hash_transactions(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        for transaction in &self.transactions {
            let id = &transaction.id;
            hasher.update(id);
        }
        format!("{:x}", hasher.finalize()).into()
    }
}
