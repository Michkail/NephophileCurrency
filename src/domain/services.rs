use super::entities::{Block, Transaction};
use sha2::{Sha256, Digest};

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![],
            pending_transactions: vec![],
            difficulty: 4,
        }
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            previous_hash: String::from("0"),
            timestamp: Self::current_timestamp(),
            transactions: vec![],
            nonce: 0,
            hash: String::new(),
        };

        let genesis_block = self.mine_block_with_nonce(genesis_block);
        self.blocks.push(genesis_block);
    }

    fn current_timestamp() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_block(&mut self) {
        let last_block = self.blocks.last().unwrap();
        let mut block = Block {
            index: self.blocks.len() as u64,
            previous_hash: last_block.hash.clone(),
            timestamp: Self::current_timestamp(),
            transactions: self.pending_transactions.clone(),
            nonce: 0,
            hash: String::new(),
        };

        block = self.mine_block_with_nonce(block);
        self.blocks.push(block);
        self.pending_transactions.clear();
    }

    fn mine_block_with_nonce(&self, mut block: Block) -> Block {
        while !self.is_valid_hash(&block.hash) {
            block.nonce += 1;
            block.hash = self.hash(&block);
        }
        block
    }

    fn is_valid_hash(&self, hash: &String) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty))
    }

    fn hash(&self, block: &Block) -> String {
        let block_data = format!(
            "{}{}{}{:?}{}",
            block.index, block.previous_hash, block.timestamp, block.transactions, block.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }
}
