use uuid::Uuid;
use crate::adapters::gateways::postgres::models::{Block, Transaction, User};
use crate::domain::entities::Block as DomainBlock;
use crate::domain::entities::Transaction as DomainTransaction;
use crate::domain::services::BlockchainRepository;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct PostgresRepository<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> BlockchainRepository for PostgresRepository<'a> {
    fn save_block(&self, block: &DomainBlock) {
        // Convert DomainBlock to DB Block and insert into DB
    }

    fn load_blocks(&self) -> Vec<DomainBlock> {
        // Load blocks from DB and convert to DomainBlock
        vec![]
    }
}
