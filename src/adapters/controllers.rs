use crate::domain::entities::Transaction;
use crate::application::usecases::interactor::BlockchainInteractor;

pub struct BlockchainController<R> {
    interactor: BlockchainInteractor<R>,
}

impl<R: crate::application::repositories::BlockchainRepository> BlockchainController<R> {
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
}
