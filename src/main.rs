mod webserver;
mod repository;
mod container_com;

use webserver::{MyServer};

#[tokio::main]
async fn main() {
    MyServer::run().await;
}