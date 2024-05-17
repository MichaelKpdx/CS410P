use crate::*;

//Struct for Question ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QuestionId(String);

//pub type QuestionMap = HashMap<String, Question>;

//impl FromStr for QuestionId {
//    type Err = std::io::Error;

//    fn from_str(id: &str) -> Result<Self, Self::Err> {
//        match id.is_empty() {
//            false => Ok(QuestionId(id.to_string())),
//            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
//        }
//    }
//}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}

//Struct for question containing id,title, content and tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
}

impl Question {
    pub fn new(id: String, title: String, content: String, tags: Option<HashSet<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
