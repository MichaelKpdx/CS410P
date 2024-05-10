mod answer;
mod api;
mod base_error;
mod question;
mod store;

use answer::*;
use api::*;
use base_error::*;
use question::*;
use store::*;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{ErrorKind, Seek, Write};
use std::str::FromStr;
use std::sync::Arc;
use tokio::{self, sync::RwLock};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
extern crate serde_json;
extern crate thiserror;

//impl QuestionBaseError{
//   pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
//       let error = QuestionBaseError {status, error};
//       (status, Json(error)).into_response()
//   }
//}

//pub async fn handler_index(State(Question): State<Arc<RwLock<Question>>>) -> Response{
//    match Question.read().await.get_random(){
//        Some(Question) => (StatusCode::OK, IndexTemplate::new(Question)).into_resposne(),
//        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
//    }
//}
//Main for starting the server and where the handler are called based on the http link
#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    //let apis = Router::new()
    //.route("/question",get(get_questions));
    //let store = Arc::new(RwLock::new(store));
    let apis = Router::new()
        .route("/question", get(get_questions))
        .route("/question/add", post(add_answer))
        .route("/question/delete/:id", delete(delete_question))
        .route("/question/:id", put(update_question));
    let web = Router::new().route("/", get(|| async { "Hello, World!" }))
        .route("/question", get(get_questions));
    //.route("/question",get(get_questions))
    //.with_state(store);

    //   let app = Router::new()
    //       .route("/", get(handler_index))
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, web).await.unwrap();
}
