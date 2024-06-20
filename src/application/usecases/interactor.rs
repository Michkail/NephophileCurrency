use crate::domain::entities::Transaction;
use crate::domain::services::Blockchain;
use crate::application::repositories::BlockchainRepository;

pub struct BlockchainInteractor<R: BlockchainRepository> {
    pub blockchain: Blockchain,
    pub repository: R,
}

impl<R: BlockchainRepository> BlockchainInteractor<R> {
    pub fn new(blockchain: Blockchain, repository: R) -> Self {
        BlockchainInteractor {
            blockchain,
            repository,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.blockchain.add_transaction(transaction);
    }

    pub fn mine_block(&mut self) {
        self.blockchain.mine_block();
        for block in &self.blockchain.blocks {
            self.repository.save_block(block);
        }
    }
}
