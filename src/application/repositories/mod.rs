use crate::domain::entities::Block;

pub trait BlockchainRepository {
    fn save_block(&self, block: &Block);
    fn load_blocks(&self) -> Vec<Block>;
}
