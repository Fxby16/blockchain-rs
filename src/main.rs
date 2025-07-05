use ed25519_dalek::VerifyingKey;

use crate::blockchain::{block::Block, blockchain::Blockchain, digital_signature::{gen_keys, sign_str, verify_str}, hash::{hash_block, bytes_to_hex_string, target_hash, double_sha256}, transactions::Transaction};

mod blockchain;
mod ui;

fn main() {
    let keys1 = gen_keys();
    let keys2 = gen_keys();

    let target = target_hash(16);

    let mut t1 = Transaction::new(hex::encode(keys1.1.as_bytes()), hex::encode(keys2.1.as_bytes()), 10.0, 0.1);
    t1.set_signature(sign_str(&t1.serialize_no_sign(), &keys1.0));
    let mut t2 = Transaction::new(hex::encode(keys2.1.as_bytes()), "zz".to_string(), 9.0, 0.1);
    t2.set_signature(sign_str(&t2.serialize_no_sign(), &keys2.0));

    let mut block1 = Block::new(0, bytes_to_hex_string(&double_sha256(&"0".to_string())), vec![t1]);
    let hash = hash_block(&mut block1, &target);
    block1.set_hash(bytes_to_hex_string(&hash));
    let mut block2 = Block::new(1, block1.get_hash().to_string(), vec![t2]);
    let hash = hash_block(&mut block2, &target);
    block2.set_hash(bytes_to_hex_string(&hash));

    let mut chain = Blockchain::default();
    chain.add_block(block1);
    chain.add_block(block2);

    // Verifica hash e firme
    println!("=== BLOCKCHAIN VERIFICATION ===");
    
    let block_hashes: Vec<String> = chain.get_blocks().keys().cloned().collect();

    let mut count = 0;
    for key in block_hashes {
        println!("\n--- Block {} ---", &count);
        count += 1;
        println!("Hash: {}", key);
        
        // Verifica hash del blocco
        let blocks = chain.get_blocks_mut();
        let calculated_hash = hash_block(blocks.get_mut(&key).unwrap(), &target);
        let hash_valid = calculated_hash == *hex::decode(&key).unwrap();
        println!("Hash valid: {}", hash_valid);
        
        // Verifica firme delle transazioni
        for (j, tx) in blocks.get(&key).unwrap().get_transactions().iter().enumerate() {
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


/* 
use crate::ui::state::AppState;

mod blockchain;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Blockchain Wallet")
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Blockchain Wallet",
        options,
        Box::new(|cc| {
            // Configura il tema
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::default());
            Ok(Box::<AppState>::default())
        }),
    )
}*/