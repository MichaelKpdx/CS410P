use crate::*;
//Struct for Answer ID
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(String);

//Struct for Answer containting content,Question ID and String
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: String,
    pub content: String,
    pub question_id: QuestionId,
}

//Hash maps for Question and Answer
pub type AnswerMap = HashMap<String, Answer>;
