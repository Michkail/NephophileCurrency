use crate::domain::entities::Transaction;
use crate::domain::services::Blockchain;
use crate::application::repositories::BlockchainRepository;

pub struct BlockchainInteractor<R: BlockchainRepository> {
    blockchain: Blockchain,
    repository: R,
}

impl<R: BlockchainRepository> BlockchainInteractor<R> {
    pub fn new(blockchain: Blockchain, repository: R) -> BlockchainInteractor<R> {
        BlockchainInteractor { blockchain, repository }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.blockchain.add_transaction(transaction);
    }

    pub fn mine_block(&mut self) {
        self.blockchain.mine_block();
        let last_block = self.blockchain.blocks.last().unwrap();
        self.repository.save_block(last_block);
    }
}
