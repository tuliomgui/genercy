mod webserver;
mod repository;

use std::sync::{Arc, Mutex};
use webserver::{ServerState, MyServer};

#[tokio::main]
async fn main() {
    let server_state = Arc::new(ServerState { sender: Mutex::new(vec![]) });
    MyServer::run(Arc::clone(&server_state)).await;
}