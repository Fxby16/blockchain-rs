use ed25519_dalek::VerifyingKey;

use crate::blockchain::{block::Block, blockchain::Blockchain, digital_signature::{gen_key, sign_str, verify_str}, hash::{hash_block, hash_str}, transactions::Transaction};

mod blockchain;

fn main() {
    let keys1 = gen_key();
    let keys2 = gen_key();

    let mut t1 = Transaction::new(hex::encode(keys1.1.as_bytes()), hex::encode(keys2.1.as_bytes()), 10.0, 0.1);
    t1.set_signature(sign_str(&t1.serialize_no_sign(), &keys1.0));
    let mut t2 = Transaction::new(hex::encode(keys2.1.as_bytes()), "zz".to_string(), 9.0, 0.1);
    t2.set_signature(sign_str(&t2.serialize_no_sign(), &keys2.0));

    let mut block1 = Block::new(0, hash_str("0".to_string()), vec![t1]);
    block1.set_hash(hash_block(&block1));
    let mut block2 = Block::new(1, block1.get_hash().to_string(), vec![t2]);
    block2.set_hash(hash_block(&block2));

    let mut chain = Blockchain::default();
    chain.add_block(block1);
    chain.add_block(block2);

    // Verifica hash e firme
    println!("=== BLOCKCHAIN VERIFICATION ===");
    
    for (i, block) in chain.get_blocks().iter().enumerate() {
        println!("\n--- Block {} ---", i);
        println!("Hash: {}", block.get_hash());
        
        // Verifica hash del blocco
        let calculated_hash = hash_block(block);
        let hash_valid = calculated_hash == *block.get_hash();
        println!("Hash valid: {}", hash_valid);
        
        // Verifica firme delle transazioni
        for (j, tx) in block.get_transactions().iter().enumerate() {
            println!("Transaction {}: {} -> {} (Amount: {})", 
                j, tx.get_sender(), tx.get_receiver(), tx.get_amount());
            
            if let Some(signature) = tx.get_signature() {
                let decoded_key = hex::decode(tx.get_sender()).expect("Error during decoding of public key");
        
                // Converti Vec<u8> in &[u8; 32]
                let key_bytes: &[u8; 32] = decoded_key.as_slice().try_into().expect("Wrong key length");
                
                let public_key = VerifyingKey::from_bytes(key_bytes).expect("Cannot get public key");
                let signature_valid = verify_str(&tx.serialize_no_sign(), signature, &public_key);
                        
                println!("  Signature present: Yes");
                println!("  Signature valid: {}", signature_valid.is_ok());
            } else {
                println!("  Signature present: No");
            }
        }
    }
    
    println!("\n=== CHAIN SUMMARY ===");
    println!("Total blocks: {}", chain.get_blocks().len());
}
