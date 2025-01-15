use crate::application::repositories::BlockchainRepository;
use crate::domain::entities::{Block};

pub struct BlockchainInteraction<R: BlockchainRepository> {
    repository: R,
}

impl<R: BlockchainRepository> BlockchainInteraction<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn add_block(&self, block: Block) {
        self.repository.save_block(&block);
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.repository.load_blocks()
    }
}
