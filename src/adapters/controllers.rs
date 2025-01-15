use actix_web::{HttpResponse, web};
use crate::application;

use crate::application::usecases::interactor::BlockchainInteraction;
use crate::domain::services::BlockchainRepository;
use serde::Serialize;

#[derive(Serialize)]
pub struct Block {
    // Define fields here
    pub id: u32,
    pub hash: String,
    pub data: String,
}

pub struct BlockchainController<R: application::repositories::BlockchainRepository> {
    interaction: BlockchainInteraction<R>,
    pub blockchain_repository: BlockchainRepository,
}

impl<R> BlockchainController<R>
where
    R: 'static + Send + Sync + crate::application::repositories::BlockchainRepository,
{
    pub fn new(interaction: BlockchainInteraction<R>) -> Self {
        Self { interaction, blockchain_repository: BlockchainRepository {
            blocks: vec![],
            pending_transactions: vec![],
            difficulty: 0,
        } }
    }

    pub async fn add_block(&self, _block: web::Json<Block>) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn get_blocks(&self) -> HttpResponse {
        let blocks = self.interaction.get_blocks();
        HttpResponse::Ok().json(blocks)
    }
}
