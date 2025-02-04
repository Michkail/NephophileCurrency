use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::infrastructure::database::establish_connection;
use crate::adapters::controllers::BlockchainController;
use crate::application::usecases::interactor::BlockchainInteraction;
use crate::application::repositories::repositories::PostgresRepository;

mod adapters;
mod application;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = establish_connection();
    let interaction = BlockchainInteraction::new(PostgresRepository { conn: &pool.get().unwrap() });
    let controller = BlockchainController::new(interaction);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/blocks", web::post().to(move |block| controller.add_block()))
            .route("/blocks", web::get().to(move || controller.get_blocks()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
