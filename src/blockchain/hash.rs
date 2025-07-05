use sha2::{Sha256, Digest};
use num_bigint::BigUint;

use crate::blockchain::block::Block;

pub fn bytes_to_hex_string(bytes: &[u8; 32]) -> String {
    hex::encode(bytes)
}

pub fn hash_block(block: &mut Block, target: &[u8; 32]) -> [u8; 32] {
    let mut block_str = block.serialize();
    let mut hash;

    let mut count : u64 = 0;

    loop {
        hash = double_sha256(&block_str);
        println!("{}", hex::encode(&hash));
        count += 1;

        if hash_meets_target(&hash, target) {
            break;
        }

        block.inc_nonce();
        block_str = block.serialize();
    }

    println!("Generated {} hashes for block {}", count, bytes_to_hex_string(&hash));

    hash
}

pub fn double_sha256(input: &String) -> [u8; 32] {
    let first = Sha256::digest(input);
    let second = Sha256::digest(&first);
    let mut result = [0u8; 32];
    result.copy_from_slice(&second);
    result
}

pub fn target_hash(difficulty: u32) -> [u8; 32] {
    let mut target = [0xffu8; 32];

    let full_bytes = (difficulty / 8) as usize;
    let remaining_bits = (difficulty % 8) as u8;

    // Metti a 0 i byte significativi
    for i in 0..full_bytes {
        target[i] = 0x00;
    }

    // Se ci sono bit parziali, calcolali nel primo byte utile
    if full_bytes < 32 && remaining_bits > 0 {
        target[full_bytes] = 0xff >> remaining_bits;
    }

    target
}

fn hash_meets_target(hash: &[u8; 32], target: &[u8; 32]) -> bool {
    let hash_num = BigUint::from_bytes_be(hash);
    let target_num = BigUint::from_bytes_be(target);

    hash_num <= target_num
}