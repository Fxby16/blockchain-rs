use crate::blockchain::transactions::Transaction;
use crate::blockchain::timestamp::get_timestamp;

pub struct Block {
    height: u64,
    timestamp: u64,
    prev_hash: String,
    transactions: Vec<Transaction>,
    nonce: u32,
    hash: String
}

impl Block {
    pub fn new(height: u64, prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = get_timestamp();
        
        Block {
            height,
            timestamp,
            prev_hash,
            transactions,
            nonce: 0,
            hash: String::new()
        }
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_prev_hash(&self) -> &String {
        &self.prev_hash
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn inc_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn serialize(&self) -> String {
        format!(
            "{{\"height\":{},\"timestamp\":{},\"prev_hash\":\"{}\",\"transactions\":[{}],\"nonce\":{}}}",
            self.height,
            self.timestamp,
            self.prev_hash,
            self.transactions.iter().map(|tx| tx.serialize()).collect::<Vec<_>>().join(","),
            self.nonce
        )
    }
}