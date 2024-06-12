use crate::*;

#[derive(Properties,Clone,PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: String,
    pub title: String,
    pub content: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
}

impl QuestionStruct{
    pub async fn get_question(key: Option<String>) -> Msg{
        let host = include_str!("../api-url.txt").trim();
        let request = match &key{
            None => format!(
                "{}/api/v1/question",
                host,
            ),
            Some(ref key) => format!(
                "{}/api/vi/joke/{}",
                host,
                key,
            ),
            };
            let response = http::Request::get(&request).send().await;
            match response {
                Err(e) => Msg::GotQuestion(Err(e)),
                Ok(data) => Msg::GotQuestion(data.json().await),
            }
        }
    }


#[derive(Properties,Clone,PartialEq,serde::Deserialize)]
pub struct QuestionProps{
    pub question: QuestionStruct,
}

pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(",")
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class = "question">
            <span>{"QandA!"}</span><br/>
            <span>{question.id.clone()}</span><br/>
            <span>{question.title.clone()}</span><br/>
            <span>{question.content.clone()}</span><br/>
            <span>{question.answer.clone()}</span><br/>
        </div>

        {format!("[id: {}", &question.id)}
        if let Some(ref tags) = question.tags {
            {format!("; tags: {}", &format_tags(tags))}
        }
  
        
        {"]"}

    </>}
}