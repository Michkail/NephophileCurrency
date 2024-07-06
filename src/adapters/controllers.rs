use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::application::usecases::BlockchainInteractor;
use crate::adapters::gateways::postgres::models::Block;

pub struct BlockchainController<R> {
    interactor: BlockchainInteractor<R>,
}

impl<R> BlockchainController<R>
where
    R: 'static + Send + Sync + crate::application::repositories::BlockchainRepository,
{
    pub fn new(interactor: BlockchainInteractor<R>) -> Self {
        Self { interactor }
    }

    pub async fn add_block(&self, block: web::Json<Block>) -> HttpResponse {
        // Convert web::Json<Block> to domain Block and call interactor
        HttpResponse::Ok().finish()
    }

    pub async fn get_blocks(&self) -> HttpResponse {
        let blocks = self.interactor.get_blocks();
        HttpResponse::Ok().json(blocks)
    }
}
