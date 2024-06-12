use crate::*;

//Handler for adding a question
/* #[allow(unused_variables, dead_code)]
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.add_q(question) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
} */
/* //Handler for adding an answer
#[allow(unused_variables)]
pub async fn add_answer(
    State(store): State<Arc<RwLock<Store>>>,
    Json(answer): Json<Answer>,
) -> Response {
    match store.write().await.add_a(answer) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
} */
//Handler for updating a question
/* #[allow(unused_variables)]
pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(question_id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.update(&question_id, question) {
        Ok(_) => StatusCode::OK.into_response(),
        //Some(q) => *q = question,
        Err(e) => "Error in update".into_response(),
        // Err(QuestionBaseErr::QuestionUnprocessable(e)) => QuesitonBaseError::response(
        //     StatusCode::UNPROCESSABLE_ENTITY,
        //     QuestionBaseErr::QuestionUnprocessable(e),
        //)
    } */

//StatusCode::OK.into_response()
//}
//Handler for deleting a question
/* #[allow(unused_variables)]
pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(question_id): Path<String>,
) -> Response {
    match store.write().await.delete(&question_id) {
        //Ok(()) =>
        Ok(()) => return "Item Deleted".into_response(),
        Err(e) => StatusCode::OK.into_response(),
        //Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
} */
//Handler for getting a question
/* #[allow(unused_variables)]
pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    //State(store),
    Path(question_id): Path<String>
    //Path(question_id): Path<String>,
) -> Response {
    match store.read().await.
} */

//Struct that calls index.html
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: Option<&'a Question>,
    tags: Option<String>,
    error: Option<String>,
}
#[allow(dead_code)]
impl<'a> IndexTemplate<'a> {
    fn question(question: &'a Question) -> Self {
        Self {
            question: Some(question),
            tags: question.tags.as_ref().map(format_tags),
            error: None,
        }
    }

    fn error(error: String) -> Self {
        Self {
            question: None,
            tags: None,
            error: Some(error),
        }
    }
}
//struct for adding a question parameters
#[derive(Deserialize)]
pub struct AddParams {
    id: String,
    new_title: String,
    new_content: String,
    new_answer: String,
    new_tags: Option<String>,
}

//struct for id parameters
#[derive(Deserialize)]
pub struct IndexParams {
    id: Option<String>,
}

//parses tags based on ','
fn parse_tags(tags: Option<String>) -> Option<HashSet<String>> {
    let tags = tags?;
    if tags.is_empty() {
        return None;
    }
    let tags: HashSet<String> = tags.split(',').map(str::trim).map(str::to_string).collect();
    if tags.is_empty() {
        None
    } else {
        Some(tags)
    }
}

//Handler for getting a random question
pub async fn handler_random(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<IndexParams>,
) -> Response {
    let store = store.read().await;

    let question = if let Some(id) = params.id {
        store.get(&id).await
    } else {
        match store.get_random().await {
            Ok(question) => return IndexTemplate::question(&question).into_response(),
            e => e,
        }
    };

    match question {
        Ok(question) => (StatusCode::OK, IndexTemplate::question(&question)).into_response(),
        Err(QuestionBaseErr::QuestionDoesNotExist(_id)) => {
            "error in handler random".into_response()
        }
        Err(_e) => "internal server error 2".into_response(),
    }
}

//Struct that calls addQuestion.html
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "addQuestion.html")]
pub struct AddTemplate {
    stylesheet: &'static str,
    //error: Option<String>,
}

impl AddTemplate {
    fn new() -> Self {
        Self {
            stylesheet: "/addQuestion.css",
        }
    }
}

//Handler that calls AddTemplate to call addQuestion.html
pub async fn handler_tell() -> Response {
    //  let error: Option<String> = "session_error".unwrap_or(None).clone;
    (StatusCode::OK, AddTemplate::new()).into_response()
}

//Handler for deleting a question
pub async fn handler_delete(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<IndexParams>,
) -> Response {
    let id: &str = params.id.as_deref().unwrap();
    match store.write().await.delete(id).await {
        Ok(()) => "question deleted".into_response(),
        _err => "error in delete".into_response(),
    }
}

//Handler for adding a question
pub async fn handler_add(
    //    State(appstate): HandlerAppState,
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<AddParams>,
    //    session: Session,
) -> Response {
    let question = Question {
        id: params.id.clone(),
        title: params.new_title,
        answer: params.new_answer,
        content: params.new_content,
        tags: parse_tags(params.new_tags),
    };

    println!("ID: {}", question.id);
    println!("TITLE: {}", question.title);
    println!("CONTENT: {}", question.content);
    println!("TAGS: {:?}", question.tags);

    let mut store = store.write().await;
    match store.add(question).await {
        Ok(()) => "question added".into_response(),
        _err => "Error in handler add".into_response(),
    }
}

//API get random for yew app
pub async fn question(State(store): State<Arc<RwLock<Store>>>,) -> Response {
    match store.read().await.get_random().await {
        Ok(question) => question.into_response(),
        _err => "No content in question".into_response(),
    }
}

//Template that calls updateQuestion.html
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "updateQuestion.html")]
pub struct UpdateTemplate {
    stylesheet: &'static str,
    //error: Option<String>,
}

impl UpdateTemplate {
    fn new() -> Self {
        Self {
            stylesheet: "/addQuestion.css",
        }
    }
}

//Handler that calls UpdateTemplate to call updateQuestion.html
pub async fn handler_rewrite() -> Response {
    //  let error: Option<String> = "session_error".unwrap_or(None).clone;
    print!("In handler rewrite");
    (StatusCode::OK, UpdateTemplate::new()).into_response()
}

//Handler that calls update question
pub async fn handler_update(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<AddParams>,
    //    Json(question): Json<Question>,
) -> Response {
    let question = Question {
        id: params.id.clone(),
        title: params.new_title,
        answer: params.new_answer,
        content: params.new_content,
        tags: parse_tags(params.new_tags),
    };

    //let update_question = Question::new(params.id,params.new_title.clone(),params.new_content,parse_tags(params.new_tags));

    //drop(question);
    //let id:&str = params.id.as_deref().unwrap();
    //let mut store = store.write().await;
    match store
        .write()
        .await
        .update(&question.id, question.clone())
        .await
    {
        Ok(()) => "question updated".into_response(),
        Err(_e) => "Error in update".into_response(),
    }
}
//INSERT INTO questions(id,title,content,answer)
//VALUES('color','color','Whats your favorite color','Blue'),
//('first','first question','what is this class', 'CS 410'),
//('second','second question','what is your name','Michael');

//INSERT INTO tags(id,tags)
//VALUES('color', 'color, question'),
//('first','first,school'),
//('second','name,second');
