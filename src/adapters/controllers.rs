use crate::domain::entities::{Block, Transaction};
use crate::application::usecases::interactor::BlockchainInteractor;
use crate::application::repositories::BlockchainRepository;

pub struct BlockchainController<R: BlockchainRepository> {
    interactor: BlockchainInteractor<R>,
}

impl<R: BlockchainRepository> BlockchainController<R> {
    pub fn new(interactor: BlockchainInteractor<R>) -> BlockchainController<R> {
        BlockchainController { interactor }
    }

    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: u64) {
        let transaction = Transaction { sender, receiver, amount };
        self.interactor.add_transaction(transaction);
    }

    pub fn mine_block(&mut self) {
        self.interactor.mine_block();
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.interactor.blockchain.blocks
    }
}
