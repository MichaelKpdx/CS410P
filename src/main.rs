
use axum::{
    routing::get,
    Router,
};
#[tokio::main]
async fn main() {
    let web = Router::new().route("/", get(||async{"Hello, World!"}));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,web).await.unwrap();
  
}
