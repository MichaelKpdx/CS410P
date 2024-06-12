//mod answer;
mod api;
mod base_error;
mod question;
mod store;

#[allow(unused_imports)]
//use answer::*;
use api::*;
use askama::Template;
use base_error::*;
use question::*;
use std::collections::HashSet;
use std::error::Error;
use store::*;

//#[warn(unused_imports)]
use serde::{Deserialize, Serialize};
//#[warn(unused_imports)]
//#[allow(unused_imports)]
//use serde_json::from_str;
//use std::collections::HashMap;
//use std::fs::File;
//use std::io::Error;
//use std::io::{ErrorKind, Seek, Write};
//use std::str::FromStr;
use std::sync::Arc;
use tokio::{self, sync::RwLock};

#[allow(unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::{StatusCode,Method},
    response::{IntoResponse, Redirect, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use tower_http::cors;
#[allow(unused_imports)]
use sqlx::{
    self,
    postgres::{PgConnection, PgPool, PgRow, Postgres},
    Pool, Row,
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
    println!("PROGRAM STARTED 4");


    let cors = cors::CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(cors::Any);
    //let apis = Router::new()
    //.route("/question",get(get_questions));
    let apis = Router::new()
    .route("/question",get(question));

    let store = Store::new().await.unwrap_or_else(|e| {
        tracing::error!("store: {}", e);
        println!("PROGRAM IS EXITING 2");
        std::process::exit(3);
    });
    let store = Arc::new(RwLock::new(store));

    //    let session_store = MemoryStore::default();
    //    let session_layer = SessionManagerLayer::new(session_store)
    //        .with_secure(false)
    //        .with_expiry(Expiry::OnSessionEnd);
    //  let store = Arc::new(RwLock::new(store));
    /* let apis = Router::new()
    .route("/question", get(get_questions))
    .route("/question/add", post(add_answer))
    .route("/question/delete/:id", delete(delete_question))
    .route("/question/:id", put(update_question)); */
    println!("PROGRAM IS RUNNING");

    let web = Router::new()
        .route("/random", get(handler_random))
        .route("/index.html", get(handler_random))
        .route("/add", get(handler_add))
        .route("/tell", get(handler_tell))
        .route("/delete", get(handler_delete))
        .route("/update", get(handler_rewrite))
        .route("/rewrite", get(handler_update))
        .nest("/api/v1",apis)
        .layer(cors)
        // .layer(session_layer)
        .with_state(store);

    println!("PROGRAM AFTER WEB");

    //.route("/question",get(get_questions))
    //.with_state(store);

    //   let app = Router::new()
    //       .route("/", get(handler_index))
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("PROGRAM AFTER LISTENER");

    axum::serve(listener, web).await.unwrap();

    println!("PROGRAM SERVER");
}
