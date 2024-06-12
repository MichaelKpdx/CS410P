use crate::*;

//Struct for store containing file both hashmaps, question and answer
#[allow(dead_code)]
#[derive(Debug)]
pub struct Store(pub Pool<Postgres>);
//pub struct Store {
//    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
//    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
//    file: File,
//questionmap: QuestionMap,
//  answermap: AnswerMap,
//}

//type QuestionMap = HashMap<
#[allow(dead_code)]
impl Store {

    //Goes to a question in the database then returns the question
    async fn to_question(&self, row: &PgRow) -> Result<Question, sqlx::Error> {
        let id = row.get("id");
        let tags = sqlx::query(r#"SELECT tag FROM tags WHERE id = $1"#)
            .bind(&id)
            .fetch_all(&self.0)
            .await?;
        let tags: HashSet<String> = tags.iter().map(|row| row.get("tag")).collect();
        let tags = if tags.is_empty() { None } else { Some(tags) };
        Ok(Question {
            id,
            title: row.get("title"),
            content: row.get("content"),
            answer: row.get("answer"),
            tags,
        })
    }

    //Inserts tags using sql
    async fn insert_tags(
        tx: &mut PgConnection,
        id: &str,
        tags: &Option<HashSet<String>>,
    ) -> Result<(), sqlx::Error> {
        if let Some(tags) = tags {
            for tag in tags {
                sqlx::query(r#"INSERT INTO tags (id, tag) VALUES ($1, $2);"#)
                    .bind(id)
                    .bind(tag)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        Ok(())
    }

    //Connects store to the postgresql database
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        use std::env::var;

        let pwf = var("PG_PASSWORDFILE")?;
        let password = std::fs::read_to_string(pwf)?;
        let url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER")?,
            password.trim(),
            var("PG_HOST")?,
            var("PG_DBNAME")?,
        );
        let pool = PgPool::connect(&url).await?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Cannot run migration");
        println!("AFTER MIGRATION");

        Ok(Store(pool))
    }
    //Store {
    //    questions: Arc::new(RwLock::new(Self::init())),
    //    answers: Arc::new(RwLock::new(HashMap::new())),
    //    file: File::create("questions.json").expect("REASON"),
    //    questionmap: QuestionMap::new(),
    //    answermap: AnswerMap::new(),
    // }

    //Gets a random question from the database using sql
    pub async fn get_random(&self) -> Result<Question, QuestionBaseErr> {
        let row = sqlx::query(r#"SELECT * FROM questions ORDER BY RANDOM () LIMIT 1;"#)
            .fetch_one(&self.0)
            .await?;

        let question = self.to_question(&row).await?;
        Ok(question)
    }

    //Adds a question to the database using sql
    pub async fn add(&mut self, question: Question) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        let result = sqlx::query(
            r#"INSERT INTO questions
            (id, title, answer, content)
            VALUES ($1, $2, $3, $4);"#,
        )
        .bind(&question.id)
        .bind(&question.title)
        .bind(&question.answer)
        .bind(&question.content)
        .execute(&mut *tx)
        .await;
        result.map_err(|e| {
            if let sqlx::Error::Database(ref dbe) = e {
                if let Some("23505") = dbe.code().as_deref() {
                    return QuestionBaseErr::QuestionExists(question.id.to_string());
                }
            }
            QuestionBaseErr::DatabaseError(e.to_string())
        })?;
        //  Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }

    //Gets a question from the database using sql
    pub async fn get<'a>(&self, index: &str) -> Result<Question, QuestionBaseErr> {
        let row = sqlx::query(r#"SELECT * FROM questions WHERE id = $1;"#)
            .bind(index)
            .fetch_one(&self.0)
            .await?;
        let question = self.to_question(&row).await?;
        Ok(question)
    }

    //Deletes a question from the database using sql
    pub async fn delete(&mut self, index: &str) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        sqlx::query(r#"DELETE FROM questions WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        Ok(tx.commit().await?)
    }

    //Updates a question from the database using sql
    pub async fn update(&mut self, index: &str, question: Question) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        let q = sqlx::query(
            r#"UPDATE questions
            SET title = $2, answer = $3, content = $4
            WHERE id = $1;"#,
        );
        q.bind(&question.id)
            .bind(&question.title)
            .bind(&question.answer)
            .bind(&question.content)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }
}
// fn init() -> HashMap<QuestionId, Question> {
//     let file = include_str!("../questions.json");
//     serde_json::from_str(file).expect("can't read questions.json")
// }
//Writes question to data base
/*     fn write_question(&mut self) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(&self.questionmap).unwrap();
    self.file.rewind()?;
    self.file.set_len(0)?;
    self.file.write_all(json.as_bytes())?;
    self.file.sync_all()
} */
//deletes question from database
/*     pub fn delete(&mut self, index: &str) -> Result<(), QuestionBaseErr> {
    //   if !self.jokemap.contains_key(index) {
    //       return Err(QuestionBaseErr::QuestionDoesNotExist(index.to_string()));
    //   }
    self.questionmap.remove(index);
    self.write_question()?;
    Ok(())
} */
//Updates a question in the database
/*     pub fn update(
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
} */
//Adds question to the data base
/*     pub fn add_q(&mut self, question: Question) -> Result<(), QuestionBaseErr> {
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
} */
