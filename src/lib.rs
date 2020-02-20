use csr::Caesar;
use seed::{prelude::*, *};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    pub text: String,
    pub processed: Option<String>,
    pub key: String,
    pub caesar: Caesar,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            text: String::new(),
            processed: None,
            key: String::new(),
            caesar: Caesar::new(0),
        }
    }
}

#[derive(Clone)]
enum Msg {
    Input(String),
    Shift(String),
    Encrypt,
    Decrypt,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Input(s) => model.text = s,
        Msg::Shift(k) => model.key = k,
        Msg::Encrypt => {
            model.caesar = update_caesar(&model.key);
            model.processed = Some(model.caesar.encrypt(model.text.clone()));
        }
        Msg::Decrypt => {
            model.caesar = update_caesar(&model.key);
            model.processed = Some(model.caesar.decrypt(model.text.clone()));
        }
    }
}

fn update_caesar(key: &str) -> Caesar {
    Caesar::new(match key.parse::<u8>() {
        Ok(k) => k,
        Err(e) => {
            error!(format!("Invalid shift of: {}", key));
            error!(format!("Error: {}", e));
            0
        }
    })
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        style! {St::Margin => "auto"; St::TextAlign => "center"},
        h1!["Caesar cipher"],
        div![
            h2!["Input:"],
            input![input_ev(Ev::Input, Msg::Input)],
            h2!["Shift:"],
            input![input_ev(Ev::Input, Msg::Shift)],
        ],
        div![
            button![simple_ev(Ev::Click, Msg::Encrypt), "Encrypt text"],
            button![simple_ev(Ev::Click, Msg::Decrypt), "Decrypt text"],
        ],
        if let Some(text) = &model.processed {
            h2![format!("Processed text: {}", text)]
        } else {
            empty![]
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
