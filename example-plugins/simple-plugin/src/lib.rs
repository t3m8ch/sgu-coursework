use extism_pdk::*;
use plugin_sdk::{
    button,
    elements::{button, fragment, row, text, text_input, FontWeight, TextSize},
    fragment, row, text, text_input, Action, PluginMetadata, StateInput, UINode, Version,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct State {
    answer: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        State { answer: None }
    }
}

#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    Ok(Json(PluginMetadata {
        name: "simple-plugin".to_string(),
        display_name: "Simple Plugin".to_string(),
        description: "A simple plugin".to_string(),
        version: Version {
            major: 0,
            minor: 1,
            patch: 0,
        },
    }))
}

#[plugin_fn]
pub fn ui(Json(state): Json<State>) -> FnResult<Json<UINode>> {
    Ok(Json(fragment!(&[
        row!(&[text!(
            "Теория графов",
            size = TextSize::Large,
            weight = FontWeight::Bold
        )]),
        row!(&[text!(
            "Что такое граф?",
            size = TextSize::Large,
            weight = FontWeight::Medium
        )]),
        row!(&[match state.answer {
            Some(answer) => text!(&format!("Вы ответили на вопрос: {}", answer)),
            None => text_input!(id = "answer", placeholder = "Введите ответ"),
        }]),
        row!(&[button!(
            &[text!("Следующий вопрос")],
            on_click_event = "next_question"
        )])
    ])))
}

#[plugin_fn]
pub fn state(
    Json(StateInput { action, old_state }): Json<StateInput<State>>,
) -> FnResult<Json<State>> {
    Ok(Json(match action {
        Action::Mount => State::default(),
        Action::Event { event, data } => match event.as_str() {
            "next_question" => State {
                answer: data.text_inputs.get("answer").cloned(),
            },
            _ => match old_state {
                Some(old_state) => old_state,
                None => State::default(),
            },
        },
    }))
}
