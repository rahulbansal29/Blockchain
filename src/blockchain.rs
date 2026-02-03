use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
        );

        Blockchain {
            chain: vec![genesis_block],
        }
    }
}
impl Blockchain {
    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();

        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
    }
}

impl Blockchain {
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
            );

            if current.hash != recalculated_hash {
                return false;
            }
        }
        true
    }
}
