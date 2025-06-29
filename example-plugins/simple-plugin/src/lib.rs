use extism_pdk::*;
use plugin_sdk::elements::*;
use plugin_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum State {
    Welcome,
    Quiz {
        question_idx: usize,
        answers: Vec<String>,
    },
    Completed {
        score: u32,
    },
}

impl Default for State {
    fn default() -> Self {
        State::Welcome
    }
}

#[derive(Debug)]
pub struct Question {
    pub text: &'static str,
    pub answers: &'static [QuestionOption],
}

#[derive(Debug)]
pub struct QuestionOption {
    pub value: &'static str,
    pub text: &'static str,
    pub correct: bool,
}

static QUIZ: [Question; 5] = [
    Question {
        text: "Какой заголовочный файл нужно подключить для использования std::cout?",
        answers: &[
            QuestionOption {
                value: "a",
                text: "#include <stdio.h>",
                correct: false,
            },
            QuestionOption {
                value: "b",
                text: "#include <iostream>",
                correct: true,
            },
            QuestionOption {
                value: "c",
                text: "#include <string>",
                correct: false,
            },
            QuestionOption {
                value: "d",
                text: "#include <vector>",
                correct: false,
            },
        ],
    },
    Question {
        text: "Как правильно объявить целочисленную переменную со значением 42?",
        answers: &[
            QuestionOption {
                value: "a",
                text: "int x = 42;",
                correct: true,
            },
            QuestionOption {
                value: "b",
                text: "integer x = 42;",
                correct: false,
            },
            QuestionOption {
                value: "c",
                text: "var x = 42;",
                correct: false,
            },
            QuestionOption {
                value: "d",
                text: "int x := 42;",
                correct: false,
            },
        ],
    },
    Question {
        text: "Что выведет следующий код?\nint x = 5;\nint* p = &x;\ncout << *p;",
        answers: &[
            QuestionOption {
                value: "a",
                text: "Адрес переменной x",
                correct: false,
            },
            QuestionOption {
                value: "b",
                text: "5",
                correct: true,
            },
            QuestionOption {
                value: "c",
                text: "Ошибка компиляции",
                correct: false,
            },
            QuestionOption {
                value: "d",
                text: "Случайное значение",
                correct: false,
            },
        ],
    },
    Question {
        text: "Какой цикл выполнится ровно 10 раз?",
        answers: &[
            QuestionOption {
                value: "a",
                text: "for(int i = 0; i <= 10; i++)",
                correct: false,
            },
            QuestionOption {
                value: "b",
                text: "for(int i = 1; i <= 10; i++)",
                correct: true,
            },
            QuestionOption {
                value: "c",
                text: "for(int i = 0; i < 11; i++)",
                correct: false,
            },
            QuestionOption {
                value: "d",
                text: "while(i < 10)",
                correct: false,
            },
        ],
    },
    Question {
        text: "Какое ключевое слово используется для наследования в C++?",
        answers: &[
            QuestionOption {
                value: "a",
                text: "extends",
                correct: false,
            },
            QuestionOption {
                value: "b",
                text: "inherits",
                correct: false,
            },
            QuestionOption {
                value: "c",
                text: "public",
                correct: true,
            },
            QuestionOption {
                value: "d",
                text: "derives",
                correct: false,
            },
        ],
    },
];

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
    Ok(Json(fragment!(&[match state {
        State::Welcome => welcome_page(),
        State::Quiz {
            question_idx,
            answers: _,
        } => quiz_page(question_idx),
        State::Completed { score } => completed_page(score),
    }])))
}

fn welcome_page() -> UINode {
    fragment!(&[row!(&[
        text!(
            "Добро пожаловать в викторину по C++",
            weight = FontWeight::Bold,
            size = TextSize::Huge
        ),
        button!(&[text!("Начать")], on_click_event = "start_quiz")
    ])])
}

fn quiz_page(question_idx: usize) -> UINode {
    let options = QUIZ[question_idx]
        .answers
        .iter()
        .map(|answer| RadioOption::new(answer.value, answer.text))
        .collect::<Vec<_>>();

    fragment!(&[row!(&[
        radio_group!(
            &options.clone(),
            id = "options",
            title = QUIZ[question_idx].text
        ),
        button!(&[text!("Ответить")], on_click_event = "submit_answer")
    ])])
}

fn completed_page(score: u32) -> UINode {
    fragment!(&[row!(&[
        text!(
            &format!("Поздравляем! Вы набрали {} баллов", score),
            weight = FontWeight::Bold,
            size = TextSize::Huge
        ),
        button!(&[text!("Начать заново")], on_click_event = "start_quiz")
    ])])
}

#[plugin_fn]
pub fn state(
    Json(StateInput { action, old_state }): Json<StateInput<State>>,
) -> FnResult<Json<State>> {
    Ok(Json(match action {
        Action::Mount => State::default(),
        Action::Event { event, data } => match event.as_str() {
            "start_quiz" => State::Quiz {
                question_idx: 0,
                answers: vec![],
            },
            "submit_answer" => {
                if let Some(State::Quiz {
                    question_idx,
                    answers,
                }) = old_state
                {
                    let new_answer_value = data.radio_groups["options"].clone();

                    if question_idx >= QUIZ.len() - 1 {
                        State::Completed {
                            score: calculate_score(&answers),
                        }
                    } else {
                        State::Quiz {
                            question_idx: question_idx + 1,
                            answers: vec![answers, vec![new_answer_value]].concat(),
                        }
                    }
                } else {
                    State::default()
                }
            }
            _ => State::default(),
        },
    }))
}

fn calculate_score(answers: &[String]) -> u32 {
    let right_count = answers
        .iter()
        .zip(QUIZ.iter().map(|option| {
            option
                .answers
                .iter()
                .find(|answer| answer.correct)
                .unwrap()
                .value
        }))
        .filter(|(actual, exp)| ***actual == **exp)
        .count() as u32;

    (f64::from(right_count) / f64::from(QUIZ.len() as u32) * 100.0).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let answers = vec![
            "b".to_string(),
            "a".to_string(),
            "b".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        assert_eq!(calculate_score(&answers), 100);
    }
}
