use sha2::{Sha256, Digest};

use crate::blockchain::block::Block;

pub fn hash_block(block : &Block) -> String {
    let block_str = block.serialize();
    
    hash_str(block_str)
}

pub fn hash_str(str : String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();

    hex::encode(result)
}