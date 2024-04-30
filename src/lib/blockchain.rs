use std::collections::HashMap;

use sled::Result;

use crate::{
    transaction::{Transaction, TxOutput},
    Block,
};

pub struct Blockchain {
    tip: String,
    db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        let db = sled::open("db/blocks")?;
        let tip = match db.get("l")? {
            Some(last) => String::from_utf8(last.to_vec()).unwrap(),
            None => {
                panic!("No genesis block, you must create one first.");
            }
        };
        db.flush()?;
        Ok(Blockchain { tip, db })
    }

    pub fn create_blockchain(address: &str) -> Result<Blockchain> {
        let db = sled::open("db/blocks")?;
        let tip = match db.get("l")? {
            Some(_) => {
                panic!("Genesis block already exists");
            }
            None => {
                let coinbase_transaction = Transaction::new_coinbase_tx(address, "");
                let genesis = Block::new(vec![coinbase_transaction], "");
                db.insert("l", genesis.hash.as_bytes())?;
                db.insert(genesis.hash.clone(), genesis.serialize().as_bytes())?;
                genesis.hash
            }
        };
        db.flush()?;
        Ok(Blockchain { tip, db })
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<Block> {
        let last_serialized_block =
            String::from_utf8(self.db.get(&self.tip)?.unwrap().to_vec()).unwrap();
        let last_block = Block::deserialize(&last_serialized_block);
        println!("last hash: {}", last_block.hash);

        let new_block = Block::new(transactions, &last_block.hash);
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

    pub fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut unspent_txs: Vec<Transaction> = Vec::new();
        let mut spend_txos: HashMap<String, Vec<i64>> = HashMap::new();

        // 区块顺序由新到旧遍历
        for block in self.iter() {
            for tx in block.transactions {
                for out_idx in 0..tx.v_out.len() {
                    if let Some(spend_out_idxs) = spend_txos.get(&tx.id) {
                        if spend_out_idxs.contains(&(out_idx as i64)) {
                            continue;
                        }
                    }

                    // 如果输出可以被提供的地址解锁，并且之前没有被花费(新区块的输出确定是未被花费的)，则将这个交易添加到 unspentTXs 中
                    if tx.v_out[out_idx].can_be_unlocked_with(address) {
                        unspent_txs.push(tx.clone());
                    }
                }

                if !tx.is_coinbase() {
                    for _in in tx.v_in {
                        if _in.can_unlock_output_with(address) {
                            let in_tx_id = _in.tx_id.clone();
                            match spend_txos.get_mut(&in_tx_id) {
                                Some(v) => {
                                    v.push(_in.v_out);
                                }
                                None => {
                                    spend_txos.insert(in_tx_id, vec![_in.v_out]);
                                }
                            }
                        }
                    }
                }
            }
        }
        unspent_txs
    }
    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i64,
    ) -> (i64, HashMap<String, Vec<i64>>) {
        let unspent_txs = self.find_unspent_transactions(address);

        let mut unspent_outputs: HashMap<String, Vec<i64>> = HashMap::new();
        let mut accumulated = 0;

        for tx in &unspent_txs {
            for out_idx in 0..tx.v_out.len() {
                let out = &tx.v_out[out_idx];
                if out.can_be_unlocked_with(address) && accumulated < amount {
                    accumulated += out.value;

                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => {
                            v.push(out_idx as i64);
                        }
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![out_idx as i64]);
                        }
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }

    pub fn find_utxo(&self, address: &str) -> Vec<TxOutput> {
        let mut utxos: Vec<TxOutput> = Vec::new();
        let unspent_txs = self.find_unspent_transactions(address);
        for tx in unspent_txs {
            for out in tx.v_out {
                if out.can_be_unlocked_with(address) {
                    utxos.push(out);
                }
            }
        }
        utxos
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
