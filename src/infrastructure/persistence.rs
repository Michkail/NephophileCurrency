use crate::domain::entities::Block;
use crate::application::repositories::BlockchainRepository;

pub struct FileStorage;

impl BlockchainRepository for FileStorage {
    fn save_block(&self, block: &Block) {
        // Implementasi penyimpanan blockchain ke file
    }

    fn load_blocks(&self) -> Vec<Block> {
        // Implementasi pemuatan blockchain dari file
        vec![]
    }
}
