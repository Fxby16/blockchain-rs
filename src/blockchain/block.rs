use crate::blockchain::transactions::Transaction;
use crate::blockchain::timestamp::get_timestamp;

pub struct Block {
    index : u32,
    timestamp : u64,
    prev_hash : String,
    transactions : Vec<Transaction>,
    nonce : u32,
    hash : String
}

impl Block {
    pub fn new(index: u32, prev_hash : String, transactions : Vec<Transaction>) -> Self {
        let timestamp = get_timestamp();
        
        Block {
            index,
            timestamp,
            prev_hash,
            transactions,
            nonce: 0,
            hash: String::new()
        }
    }

    pub fn set_hash(&mut self, hash : String) {
        self.hash = hash;
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn serialize(&self) -> String {
        format!(
            "{{\"index\":{},\"timestamp\":{},\"prev_hash\":\"{}\",\"transactions\":[{}],\"nonce\":{}}}",
            self.index,
            self.timestamp,
            self.prev_hash,
            self.transactions.iter().map(|tx| tx.serialize()).collect::<Vec<_>>().join(","),
            self.nonce
        )
    }
}