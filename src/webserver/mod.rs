use std::sync::{Arc, Mutex};
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, Response};
use axum::{Form, Json, Router};
use axum::extract::{State, WebSocketUpgrade};
use axum::routing::{get, post};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tera::Context;
use tower_http::services::ServeDir;

mod templater;
mod error;

use templater::Templates;

#[derive(Debug, Serialize, Deserialize)]
struct ClientData {
    name: String,
    age: i16
}

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    message: String
}

pub struct ServerState {
    pub sender: Mutex<Vec<SplitSink<WebSocket, Message>>>
}

pub struct MyServer;

impl MyServer {
    pub async fn run(server_state: Arc<ServerState>) {
        println!("Initializing...");
        let app = Router::new()
            .route("/", get(MyServer::index))
            .route("/hello", get(MyServer::hello_world))
            .route("/form", post(MyServer::form_test))
            .route("/ws", get(MyServer::ws_handler))
            .nest_service("/static", ServeDir::new("static"))
            .with_state(server_state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn index() -> Html<String> {
        let templater = Templates::get_templater();
        let context = Context::new();
        let result_str = templater.render("tera_index.html", &context).unwrap();
        Html(result_str)
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
    }
    async fn ws_handler(ws: WebSocketUpgrade, State(curr_state): State<Arc<ServerState>>) -> Response {
        println!("New client connected.");
        ws.on_upgrade(|socket| MyServer::handle_socket(socket, curr_state))
    }

    async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
        let (sender, receiver) = socket.split();
        {
            let mut vec_sender = state.sender.lock().unwrap();
            vec_sender.push(sender);
        }
        tokio::spawn(MyServer::read(receiver));
    }

    async fn read(mut receiver: SplitStream<WebSocket>) {
        while let Some(res) = receiver.next().await {
            match res {
                Ok(msg) => { Self::process_message(msg); },
                Err(error) => { println!("Ocorreu um erro: {:?}", error); }
            }
        }
    }

    fn process_message(message: Message) {
        match message {
            Message::Text(text) => {
                println!("Received message: {text}");
            }
            Message::Ping(_ping) => {
                println!("received ping.");
            },
            Message::Pong(_pong) => {
                println!("received pong.");
            },
            Message::Binary(_bin) => {
                println!("received binary.");
            },
            Message::Close(_close) => {
                println!("Client closed the connection.");
            }
        }
    }

    pub async fn write(server_state: &Arc<ServerState>, message: &String) {
        for sender in server_state.sender.lock().unwrap().iter_mut() {
            if sender.send(Message::Text(message.clone())).await.is_err() {
                println!("Client disconnected");
            }
        }
    }
}