use chrono::{Local, TimeZone};
use num::{BigUint, One};
use sha2::{Digest, Sha256};

use crate::Block;

const TARGET_BITS: u32 = 16;
const MAX_NONCE: u64 = u64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigUint,
}

impl ProofOfWork {
    pub fn new(block: Block) -> ProofOfWork {
        let mut target: BigUint = BigUint::one();
        target <<= 256 - TARGET_BITS;

        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let data = format!(
            "{}{}{}{}{}",
            self.block.prev_block_hash, self.block.data, self.block.timestamp, TARGET_BITS, nonce
        );
        data.as_bytes().to_vec()
    }

    pub fn run(&mut self) -> (u64, String) {
        let mut hasher = Sha256::new();
        let mut nonce = 0u64;
        let start = Local::now().timestamp_millis();
        println!("Mining the block containing \"{}\"", self.block.data);
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hasher = Sha256::new();
            hasher.update(&data);

            if self.target > BigUint::from_bytes_be(&hasher.clone().finalize()) {
                let end = Local::now().timestamp_millis();
                println!("Hash: {:x}", hasher.clone().finalize());
                println!("Nonce: {} Mining Duration: {}ms", nonce, end - start);
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n\n");

        (self.block.nonce, format!("{:x}", &hasher.finalize()))
    }
}
