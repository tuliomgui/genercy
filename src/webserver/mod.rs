use std::collections::{HashMap, BTreeMap};
use std::fmt::Error;
use std::sync::{Mutex, OnceLock};
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, IntoResponse, Response};
use axum::{Form, Json, Router};
use axum::extract::{Path, WebSocketUpgrade};
use axum::routing::{get, post, delete};
use axum::http::StatusCode;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tera::Context;
use tower_http::services::ServeDir;

mod templater;
mod error;
use crate::container_com::*;
use crate::container_com::image::*;

use templater::Templates;

static CONNECTED_CLIENTS: OnceLock<ServerState> = OnceLock::new();

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
    pub async fn run() {
        println!("Initializing...");
        if let None = CONNECTED_CLIENTS.get() {
            if let Err(_) = CONNECTED_CLIENTS.set(ServerState { sender: Mutex::new(vec![]) }) {
                panic!("Error trying to start the server. The list of the connected clients could not be initialized properly.");
            }
        };
        //let server_state =
        let app = Router::new()
            .route("/", get(MyServer::index))
            .route("/hello", get(MyServer::hello_world))
            .route("/hello2", get(MyServer::hello_world2))
            .route("/form", post(MyServer::form_test))
            .route("/ws", get(MyServer::ws_handler))
            .route("/container/:id/:action", get(MyServer::container_action_handler))
            .route("/image/:id", delete(MyServer::image_delete_handler))
            .route("/images", get(MyServer::images))
            .nest_service("/static", ServeDir::new("static"));
            //.with_state(server_state);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Server is running...");
        axum::serve(listener, app).await.unwrap();
    }

    async fn image_delete_handler(Path(id): Path<String>) -> impl IntoResponse {
        let id_copy = id.clone();
        match DockerDeleteImages::execute(vec![id]) {
            Ok(x) => (StatusCode::OK, Json(json!({"id": id_copy, "success": true, "message": "Image was successfully deleted."}))),
            Err(error) => (StatusCode::OK, Json(json!({"id": id_copy, "success": false, "message": error})))
        }
    }

    async fn container_action_handler(Path((id, action)): Path<(String, String)>) -> impl IntoResponse {
        let mut context = Context::new();
        context.insert("container", &HashMap::from([("ID", &id)]));
        let id_copy = id.clone();
        match action.as_str() {
            "start" => {
                match DockerStartContainers::execute(vec![id]) {
                    Ok(x) => (StatusCode::OK, Json(json!({"id": id_copy, "success": true, "message": format!("Container {} was successfully started", id_copy)}))),
                    Err(error) => (StatusCode::OK, Json(json!({"id": id_copy, "success": false, "message": error})))
                }
            }
            "stop" => {
                match DockerStopContainers::execute(vec![id]) {
                    Ok(x) => (StatusCode::OK, Json(json!({"id": id_copy, "success": true, "message": format!("Container {} was successfully stopped", id_copy)}))),
                    Err(error) => (StatusCode::OK, Json(json!({"id": id_copy, "success": false, "message": error})))
                }
            }
            _ => (StatusCode::BAD_REQUEST, Json(json!({"id": id_copy, "success": false, "message": "Bad request"}))),
        }
    }

    async fn index() -> Html<String> {
        let templater = Templates::get_templater();
        let mut context = Context::new();
        let x = DockerListAllContainers::execute(vec![]).unwrap();
        context.insert("containers", &x.output);
        let result_str = templater.render("containers.html", &context).unwrap();
        Html(result_str)
    }

    async fn images() -> Html<String> {
        let templater = Templates::get_templater();
        let mut context = Context::new();
        let x = DockerListAllImages::execute(vec![]).unwrap();
        context.insert("images", &x.output);
        let result_str = templater.render("images.html", &context).unwrap();
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

    async fn hello_world2() -> Json<Value> {
        let msg = format!("Funcionou");
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
    async fn ws_handler(ws: WebSocketUpgrade) -> Response {
        println!("New client connected.");
        ws.on_upgrade(|socket| MyServer::handle_socket(socket))
    }

    async fn handle_socket(socket: WebSocket) {
        let (sender, receiver) = socket.split();
        {
            let mut vec_sender = CONNECTED_CLIENTS.get().unwrap().sender.lock().unwrap();
            vec_sender.push(sender);
        }
        tokio::spawn(MyServer::read(receiver));
    }

    async fn read(mut receiver: SplitStream<WebSocket>) {
        while let Some(res) = receiver.next().await {
            match res {
                Ok(msg) => { Self::process_message(msg, &receiver); },
                Err(error) => { println!("Ocorreu um erro: {:?}", error); }
            }
            //receiver.
        }
    }

    fn process_message(message: Message, receiver: &SplitStream<WebSocket>) {
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
                Self::remove_disconnected_client(receiver);
            }
        }
    }

    fn remove_disconnected_client(receiver: &SplitStream<WebSocket>) {
        let mut available_clients = CONNECTED_CLIENTS.get().unwrap().sender.lock().unwrap();
        available_clients.retain(|client| !client.is_pair_of(receiver));
    }

    pub async fn write(message: &String) -> Result<(), Error> {
        {
            let mut x = CONNECTED_CLIENTS.get().unwrap().sender.lock().unwrap();
            // Broadcast message to all connected clients
            for sender in x.iter_mut() {
                if sender.send(Message::Text(message.clone())).await.is_err() {
                    println!("Client is disconnected");
                }
            }
        }
        Ok(())
    }
}
