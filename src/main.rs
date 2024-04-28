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
#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    #[error("question already exists: {0}")]
    QuestionExists(String),
    #[error("question base io failed: {0}")]
    QuestionBaseIoError(String),
    #[error("no question")]
    NoQuestion,
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("Question paylor unprocessable")]
    QuestionUnprocessable(String),
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
    }
}

#[derive(Debug)]
pub struct QuestionBaseError {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: String,
    content: String,
    question_id: QuestionId,
}

type QuestionMap = HashMap<String, Question>;
type AnswerMap = HashMap<String, Answer>;
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    file: File,
    questionmap: QuestionMap,
    answermap: AnswerMap,
}

//type QuestionMap = HashMap<
impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
            file: File::create("questions.json").expect("REASON"),
            questionmap: QuestionMap::new(),
            answermap: AnswerMap::new(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    fn write_question(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.questionmap).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
    }

    pub fn delete(&mut self, index: &str) -> Result<(), QuestionBaseErr> {
        //   if !self.jokemap.contains_key(index) {
        //       return Err(QuestionBaseErr::QuestionDoesNotExist(index.to_string()));
        //   }
        self.questionmap.remove(index);
        self.write_question()?;
        Ok(())
    }

    pub fn update(
        &mut self,
        index: &str,
        question: Question,
    ) -> Result<StatusCode, QuestionBaseErr> {
        // if !self.questions.contains_key(index) {
        //     return Err(QuestionBaseErr::NoQuestion);
        //}
        //if question.is_empty(){
        //    return Err(QuestionBaseErr::QuestionUnprocessable(index.to_string()));
        //}
        self.questionmap
            .entry(index.to_string())
            .and_modify(|x| *x = question);
        self.write_question()?;
        Ok(StatusCode::OK)
    }

    pub fn addQ(&mut self, question: Question) -> Result<(), QuestionBaseErr> {
        let id = question.id.clone();
        self.questionmap.insert(id, question);
        self.write_question()?;
        Ok(())
    }

    pub fn addA(&mut self, answer: Answer) -> Result<(), QuestionBaseErr> {
        let id = answer.id.clone();
        self.answermap.insert(id, answer);
        self.write_question()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: String, title: String, content: String, tags: Option<Vec<String>>) -> Self {
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
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}

pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.addQ(question) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
}

pub async fn add_answer(
    State(store): State<Arc<RwLock<Store>>>,
    Json(answer): Json<Answer>,
) -> Response {
    match store.write().await.addA(answer) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
}

pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.update(&questionId, question) {
        Ok(_) => StatusCode::OK.into_response(),
        //Some(q) => *q = question,
        Err(e) => "Error in update".into_response(),
        // Err(QuestionBaseErr::QuestionUnprocessable(e)) => QuesitonBaseError::response(
        //     StatusCode::UNPROCESSABLE_ENTITY,
        //     QuestionBaseErr::QuestionUnprocessable(e),
        //)
    }

    //StatusCode::OK.into_response()
}

pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
) -> Response {
    match store.write().await.delete(&questionId) {
        //Ok(()) =>
        Ok(()) => return "Item Deleted".into_response(),
        Err(e) => StatusCode::OK.into_response(),
        //Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}

pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    Path(questionId): Path<String>,
) -> Response {
    let question = Question::new(
        from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of quetsion".to_string(),
        Some(vec!["faq".to_string()]),
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
        .route("/question/:id", put(update_question));
    let web = Router::new().route("/", get(|| async { "Hello, World!" }));
    //.route("/question",get(get_questions))
    //.with_state(store);

    //   let app = Router::new()
    //       .route("/", get(handler_index))
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, web).await.unwrap();
}
