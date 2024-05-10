use crate::*;

//Struct for store containing file both hashmaps, question and answer
#[allow(dead_code)]
pub struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    file: File,
    questionmap: QuestionMap,
    answermap: AnswerMap,
}

//type QuestionMap = HashMap<
#[allow(dead_code)]
impl Store {
    pub fn new() -> Self {
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
    //Writes question to data base
    fn write_question(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.questionmap).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
    }
    //deletes question from database
    pub fn delete(&mut self, index: &str) -> Result<(), QuestionBaseErr> {
        //   if !self.jokemap.contains_key(index) {
        //       return Err(QuestionBaseErr::QuestionDoesNotExist(index.to_string()));
        //   }
        self.questionmap.remove(index);
        self.write_question()?;
        Ok(())
    }
    //Updates a question in the database
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
    //Adds question to the data base
    pub fn add_q(&mut self, question: Question) -> Result<(), QuestionBaseErr> {
        let id = question.id.clone();
        self.questionmap.insert(id, question);
        self.write_question()?;
        Ok(())
    }
    //Adds an answer to the database
    pub fn add_a(&mut self, answer: Answer) -> Result<(), QuestionBaseErr> {
        let id = answer.id.clone();
        self.answermap.insert(id, answer);
        self.write_question()?;
        Ok(())
    }
}
