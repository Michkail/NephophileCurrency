#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}
