mod webserver;

use std::sync::{Arc, Mutex};
use webserver::{ServerState, MyServer};
//use tokio::{task};

#[tokio::main]
async fn main() {
    let server_state = Arc::new(ServerState { sender: Mutex::new(vec![]) });
    MyServer::run(Arc::clone(&server_state)).await;
    //task::spawn(MyServer::run(Arc::clone(&server_state)));

    //let handle = MyServer::run(Arc::clone(&server_state));
    /*loop {
        println!("Digite sua mensagem: ");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let value: String = line.parse().unwrap();
        println!("Received: {}.", value.trim_end());
        MyServer::write(&server_state, &value).await;
        if value.trim_end().eq("quit") {
            break;
        }
    }*/

    // let t = handle.await.with_graceful_shutdown(async {});
    // t.await.unwrap();
    // println!("Fim");
    //println!("Acabou: {:?}", handle.await.unwrap());

    //MyServer::run().await;
}



/*******************************************

       WEBSOCKET SERVER CODE USING AXUM

 ********************************************/


/*//use std::fmt::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ClientData {
    name: String,
    age: i16
}

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    message: String
}

use axum::{routing::get, routing::post, Router, response::Json, extract::Form, extract::ws::WebSocketUpgrade, response::Response};
use futures_util::{SinkExt, stream::{SplitStream, SplitSink}, StreamExt};
use tower_http::services::ServeDir;
use serde_json::{Value, json};
use axum::extract::ws::{Message, WebSocket};

#[allow(dead_code)]
struct WSSender {
    sender: SplitSink<WebSocket, Message>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
                        .route("/", post(hello_world))
                        .route("/form", post(form_test))
                        .route("/ws", get(ws_handler))
                        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Fechou!");
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    println!("New client connected.");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(msg)) = receiver.next().await {
        let temp = msg.into_text().unwrap();
        println!("Received message: {temp}");
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    for i in 0..10 {
        let t = i + 1;
        if sender.send(Message::Text(String::from(format!("Async message {t}")))).await.is_err() {
            println!("Client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

async fn hello_world(axum::extract::Json(body_data): axum::extract::Json<ClientData>) -> Json<Value> {
    println!("Raw data: {body_data:?}");
    let name = &body_data.name;
    let age = &body_data.age;
    let msg = format!("Hi, {name} you are {age} years old.");
    let return_msg = ResponseMessage{message: String::from(msg)};
    Json(json!(return_msg))
}

async fn form_test(Form(form): Form<ClientData>) -> Json<Value> {
    println!("Form data: {form:?}");
    let name = &form.name;
    let age = &form.age;
    let msg = format!("Hi, {name} you are {age} years old. I got this data from a form submission.");
    let return_msg = ResponseMessage{message: String::from(msg)};
    Json(json!(return_msg))
}*/
