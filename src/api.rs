use crate::*;

//Handler for adding a question
#[allow(unused_variables, dead_code)]
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.add_q(question) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
}
//Handler for adding an answer
#[allow(unused_variables)]
pub async fn add_answer(
    State(store): State<Arc<RwLock<Store>>>,
    Json(answer): Json<Answer>,
) -> Response {
    match store.write().await.add_a(answer) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => "Error in adding question".into_response(),
    }
}
//Handler for updating a question
#[allow(unused_variables)]
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
    }

    //StatusCode::OK.into_response()
}
//Handler for deleting a question
#[allow(unused_variables)]
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
}
//Handler for getting a question
#[allow(unused_variables)]
pub async fn get_questions(
    //State(store): State<Arc<RwLock<Store>>>,
    //State(store),
    Path(question_id): Path<String>
    //Path(question_id): Path<String>,
) -> Response {
    let question = Question::new(
        from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of quetsion".to_string(),
        Some(vec!["faq".to_string()]),
    );
    question.into_response()
}
