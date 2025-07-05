use eframe::egui::ahash::{HashMap, HashSet};

use crate::blockchain::block::Block;

#[derive(Default)]
pub struct Blockchain {
    blocks: HashMap<String, Block>,
    heads_data: HashMap<String, u64>,
    heads: HashSet<String>,
    main_head: String
}

impl Blockchain {
    pub fn add_block(&mut self, block: Block) {
        let hash = block.get_hash().clone();
        let parent_hash = block.get_prev_hash().clone();
        
        if self.blocks.contains_key(&hash) {
            println!("Block already in the chain. Something is wrong");
            return;
        }

        self.blocks.insert(hash.clone(), block);
        self.heads.remove(&parent_hash);

        self.heads.insert(hash.clone());
        self.heads_data.insert(hash.clone(), self.heads_data.get(&parent_hash).unwrap_or(&0) + 1);
    
        let main_head_height = self.heads_data.get(&self.main_head).unwrap_or(&0);
        let current_head_height = self.heads_data.get(&hash).unwrap();
    
        if current_head_height > main_head_height {
            self.main_head = hash;
        }
    }

    pub fn get_blocks(&self) -> &HashMap<String, Block> {
        &self.blocks
    }

    pub fn get_blocks_mut(&mut self) -> &mut HashMap<String, Block> {
        &mut self.blocks
    }

    pub fn get_block(&self, hash: &String) -> Option<&Block> {
        self.blocks.get(hash)
    }

    pub fn get_heads(&self) -> &HashSet<String> {
        &self.heads
    }

    pub fn get_main_head(&self) -> &String {
        &self.main_head
    }

    pub fn get_heads_data(&self) -> &HashMap<String, u64> {
        &self.heads_data
    }
}