use axum::{response::Json, routing::post, Router, extract::State};
use serde::Deserialize;
use serde::Serialize;
use tower_http::cors::CorsLayer;
mod model;
mod llm_model;
use llm::models::Llama;
use  model::Conversation;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load llama model
    let mdl: Llama = llm_model::language_model();

    // build our application with a route
    let app = Router::new().route("/api/chat", post(handler)).layer(CorsLayer::permissive()).with_state(Arc::new(mdl));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Response {
    pub result: String
}

async fn handler(State(mdl): State<Arc<Llama>>,Json(chat): Json<Conversation>) -> Json<Response> {
    println!("{:#?}", chat);
    let res = llm_model::gen_conversation(&*mdl, chat);
    Json(Response{result: res})
}