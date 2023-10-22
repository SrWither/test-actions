pub mod database;
pub mod models;
pub mod routes;
pub mod blog;

use routes::router;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:8000").bind().await;
    Server::new(acceptor).serve(router()).await;
}
