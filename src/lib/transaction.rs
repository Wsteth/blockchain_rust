use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{blockchain, Blockchain};

const SUBSIDY: i64 = 10;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub v_in: Vec<TxInput>,
    pub v_out: Vec<TxOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutput {
    pub value: i64,
    /// 锁定的脚本
    pub script_pub_key: String,
}

impl TxOutput {
    pub fn can_be_unlocked_with(&self, unlock_data: &str) -> bool {
        self.script_pub_key == unlock_data
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput {
    /// 引用的交易id
    pub tx_id: String,
    /// 引用的交易中的输出索引
    pub v_out: i64,
    /// 锁定脚本，校验输出是否可以被解锁
    pub script_sig: String,
}

impl TxInput {
    pub fn can_unlock_output_with(&self, unlock_data: &str) -> bool {
        self.script_sig == unlock_data
    }
}

impl Transaction {
    pub fn new_utxo_transaction(
        from: &str,
        to: &str,
        amount: i64,
        blockchain: &Blockchain,
    ) -> Transaction {
        let mut inputs: Vec<TxInput> = Vec::new();
        let mut outputs: Vec<TxOutput> = Vec::new();
        let (acc, valid_outputs) = blockchain.find_spendable_outputs(from, amount);

        if acc < amount {
            panic!("not enough funds");
        }

        for (tx_id, outs) in valid_outputs {
            for out in outs {
                let input = TxInput {
                    tx_id: tx_id.clone(),
                    v_out: out,
                    script_sig: from.into(),
                };
                inputs.push(input);
            }
        }

        outputs.push(TxOutput {
            value: amount,
            script_pub_key: to.into(),
        });

        if acc > amount {
            outputs.push(TxOutput {
                value: acc - amount,
                script_pub_key: from.into(),
            })
        }

        let mut tx = Transaction {
            id: String::new(),
            v_in: inputs,
            v_out: outputs,
        };
        tx.set_id();
        tx
    }

    pub fn new_coinbase_tx(to: &str, data: &str) -> Transaction {
        let data = if data.is_empty() {
            format!("reward to '{}'", to)
        } else {
            data.to_string()
        };

        let txin = TxInput {
            tx_id: "".into(),
            v_out: -1,
            script_sig: data,
        };

        let txout = TxOutput {
            value: SUBSIDY,
            script_pub_key: to.into(),
        };

        let mut tx = Transaction {
            id: String::new(),
            v_in: vec![txin],
            v_out: vec![txout],
        };
        tx.set_id();
        tx
    }

    pub fn set_id(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap());
        self.id = format!("{:x}", hasher.finalize());
    }

    pub fn is_coinbase(&self) -> bool {
        self.v_in.len() == 1 && self.v_in[0].tx_id.is_empty() && self.v_in[0].v_out == -1
    }
}
