mod webserver;
mod repository;

use webserver::{MyServer};

#[tokio::main]
async fn main() {
    MyServer::run().await;
}