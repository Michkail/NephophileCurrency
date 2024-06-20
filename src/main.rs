mod application;
mod domain;
mod infrastructure;
mod adapters;

use domain::services::Blockchain;
use adapters::controllers::BlockchainController;
use application::usecases::interactor::BlockchainInteractor;
use infrastructure::persistence::FileStorage;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.create_genesis_block();

    let storage = FileStorage;
    let interactor = BlockchainInteractor::new(blockchain, storage);
    let mut controller = BlockchainController::new(interactor);

    controller.add_transaction("Alice".to_string(), "Bob".to_string(), 50);
    controller.add_transaction("Bob".to_string(), "Charlie".to_string(), 30);
    println!("Mining block...");
    controller.mine_block();

    controller.add_transaction("Charlie".to_string(), "Dave".to_string(), 20);
    println!("Mining block...");
    controller.mine_block();

    // Output atau logika lain
    println!("Current blockchain: {:?}", controller.get_blocks());
}
