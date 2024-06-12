mod question;

use question::*;

use std::collections::HashSet;
extern crate serde;
extern crate wasm_bindgen_futures;
//use gloo_console::log;
use gloo_net::http;
//use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
//use yew::functional::*;

pub type QuestionResult = Result<QuestionStruct, gloo_net::Error>;

struct App{
    question: QuestionResult,
}

pub enum Msg{
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

impl App{

    fn  refresh_question(ctx: &Context<Self>, key: Option<String>){
        let got_question = QuestionStruct::get_question(key);
        ctx.link().send_future(got_question);
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_question(ctx, None);
        let question = Err(gloo_net::Error::GlooError("Loading Question...".to_string()));
        Self { question }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            Msg::GetQuestion(key) => {
                App::refresh_question(ctx, key);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let question = &self.question;

        html! {
            <>
            <h1>{"QandA"}</h1>
          //  if false {
    
           // }
           if let Ok(ref question) = *question {
            <Question question={question.clone()}/>
           }
           if let Err(ref error) = *question {
            <div>
            <p>{error.to_string().clone()}</p>
            <p>{"UH OH"}</p>
            </div>
           }
           <div>
                <button onclick={ctx.link().callback(|_| Msg::GetQuestion(None))}>{"Anotha one!"}</button>
            </div>
           </>
        }
    }

}

fn main() {
    print!("HELLO WORLD\n");
    yew::Renderer::<App>::new().render();
}
