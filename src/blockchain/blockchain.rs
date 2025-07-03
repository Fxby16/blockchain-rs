use crate::blockchain::block::Block;

#[derive(Default)]
pub struct Blockchain {
    blocks : Vec<Block>
}

impl Blockchain {
    pub fn add_block(&mut self, block : Block) {
        self.blocks.push(block);
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}