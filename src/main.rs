use std::io::{ErrorKind,Write};
use std::io::Error;
use std::str::FromStr;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{self, sync::RwLock};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete,get,post,put},
    response::{IntoResponse, Response},
    Json, Router,
};
extern crate serde_json;
extern crate thiserror;
#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr{
    #[error("question already exists: {0}")]
    QuestionExists(String),
    #[error("question base io failed: {0}")]
    QuestionBaseIoError(String),
    #[error("no question")]
    NoQuestion,
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("Question paylor unprocessable")]
    QuestionUnprocessable(String)
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
    }
}

#[derive(Debug)]
pub struct QuestionBaseError{
    pub status: StatusCode,
    pub error: QuestionBaseErr,
}

//impl QuestionBaseError{
 //   pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
 //       let error = QuestionBaseError {status, error};
 //       (status, Json(error)).into_response()
 //   }
//}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct AnswerId(String);

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq,Hash)]
struct QuestionId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer{
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

struct Store{
    questions: Arc<RwLock<HashMap<QuestionId,Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId,Answer>>>
}
impl Store{
    fn new() -> Self {
        Store {
        questions: Arc::new(RwLock::new(Self::init())),
        answers:  Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn init() -> HashMap<QuestionId, Question>{
     let file = include_str!("../questions.json");
     serde_json::from_str(file).expect("can't read questions.json")
    }

}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Question{
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}




impl Question{
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self{
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty(){
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}

//fn write_question(&mut self) -> Result<(), std::io::Error>{
//    let json = serde_json::to_string(&self.HashMap).unwrap();
//    self.file.rewind()?;
//    self.file.sen_len(0)?;
//    self.file.write_all(json.as_bytes())?;
//    self.file.sync_all*
//}

pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    store.questions.write().await.insert (question.id.clone(), question);
    "Question added".into_response()
    //Ok()
}

pub async fn add_answer(
    State(store): State<Arc<RwLock<Store>>>,
    Json(answer): Json<Answer>,
) -> Response{
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(
            params.get("questionId").unwrap().to_string()
        ),
    };
    store.answers.write().await.insert(answer.id.clone(),answer);
    "Answer added".into_response()
    //OK(_)
}

pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.update(&questionId,question){
        //Ok(_) => 
        Some(q) => *q = question,
        None => return StatusCode::OK.into_response(),
        
       // Err(QuestionBaseErr::QuestionUnprocessable(e)) => QuesitonBaseError::response(
       //     StatusCode::UNPROCESSABLE_ENTITY,
       //     QuestionBaseErr::QuestionUnprocessable(e),
        //)
    }

    StatusCode::OK.into_response()
}

pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
) -> Response {
    match store.write().await.delete(&questionId) {
        //Ok(()) =>
        Some(_) => return "Item Deleted".into_response(), 
        None => StatusCode::OK.into_response(),
       // Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}

pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
) -> Response{
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of quetsion".to_string(),
        Some(vec!("faq".to_string())),
    );
    question.into_response()
}

//pub async fn handler_index(State(Question): State<Arc<RwLock<Question>>>) -> Response{
//    match Question.read().await.get_random(){
//        Some(Question) => (StatusCode::OK, IndexTemplate::new(Question)).into_resposne(),
//        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
//    }
//}
#[tokio::main]
async fn main() {
    //let apis = Router::new()
    //.route("/question",get(get_questions));
    //let store = Arc::new(RwLock::new(store));
    let apis = Router::new()
        .route("/question", get(get_questions))
        .route("/question/add", post(add_answer))
        .route("/question/:id", delete(delete_question))
        .route("/question/:id",put(update_question));
    let web = Router::new()
    .route("/", get(||async{"Hello, World!"}));
    //.route("/question",get(get_questions))
    //.with_state(store);

 //   let app = Router::new()
 //       .route("/", get(handler_index))
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,web).await.unwrap();
}
