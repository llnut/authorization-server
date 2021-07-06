#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use tonic::transport::Server;
use user_server::pb_user_server::PbUserServer;

pub mod user_server {
    tonic::include_proto!("user_server");
}

mod config;
mod error;
mod handler;
mod model;
mod schema;
mod service;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = match config::Config::try_from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Invalid configuration: {}", e);
            const EX_USAGE: i32 = 64;
            std::process::exit(EX_USAGE);
        }
    };
    let db_pool: config::DbPool = cfg.build_db_pool().await;
    let pb_user_server = handler::user::build_server(db_pool.clone()).await;

    println!("GreeterServer listening on {}", cfg.listen_addr);

    Server::builder()
        .add_service(PbUserServer::new(pb_user_server))
        .serve(cfg.listen_addr)
        .await?;

    Ok(())
}
