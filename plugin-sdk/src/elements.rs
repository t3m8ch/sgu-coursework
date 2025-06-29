use std::collections::HashMap;

use opt_args::opt_args;
use strum_macros::Display;

use crate::UINode;

opt_args! {
    pub fn fragment(children: &[UINode]) -> UINode {
        UINode {
            name: "fragment".to_string(),
            children: children.to_vec(),
            props: HashMap::new(),
        }
    }
}

opt_args! {
    pub fn row(children: &[UINode]) -> UINode {
        UINode {
            name: "row".to_string(),
            children: children.to_vec(),
            props: HashMap::new(),
        }
    }
}

#[derive(Display, Debug)]
pub enum FontWeight {
    #[strum(serialize = "regular")]
    Regular,

    #[strum(serialize = "medium")]
    Medium,

    #[strum(serialize = "bold")]
    Bold,
}

#[derive(Display, Debug)]
pub enum TextSize {
    #[strum(serialize = "small")]
    Small,

    #[strum(serialize = "normal")]
    Normal,

    #[strum(serialize = "large")]
    Large,

    #[strum(serialize = "huge")]
    Huge,
}

opt_args! {
    #[opt_args(shuffle)]
    pub fn text(text: &str, weight: FontWeight = FontWeight::Regular, size: TextSize = TextSize::Normal) -> UINode {
        UINode {
            name: "text".to_string(),
            children: vec![],
            props: HashMap::from([
                ("text".to_string(), text.to_string()),
                ("weight".to_string(), weight.to_string()),
                ("size".to_string(), size.to_string()),
            ]),
        }
    }
}

opt_args! {
    #[opt_args(shuffle)]
    pub fn button(children: &[UINode], on_click_event: &str = "") -> UINode {
        let mut props = HashMap::new();
        if !on_click_event.is_empty() {
            props.insert("on_click_event".to_string(), on_click_event.to_string());
        };

        UINode {
            name: "button".to_string(),
            children: children.to_vec(),
            props,
        }
    }
}

opt_args! {
    #[opt_args(shuffle)]
    pub fn text_input(id: &str = "", placeholder: &str = "") -> UINode {
        let mut props = HashMap::new();
        if !id.is_empty() {
            props.insert("id".to_string(), id.to_string());
        };
        props.insert("placeholder".to_string(), placeholder.to_string());

        UINode {
            name: "text_input".to_string(),
            children: vec![],
            props,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
}

impl RadioOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
        }
    }
}

opt_args! {
    #[opt_args(shuffle)]
    pub fn radio_group(options: &[RadioOption], id: &str = "", title: &str = "") -> UINode {
        let mut props = HashMap::new();
        if !id.is_empty() {
            props.insert("id".to_string(), id.to_string());
        }
        if !title.is_empty() {
            props.insert("title".to_string(), title.to_string());
        }

        UINode {
            name: "radio_group".to_string(),
            children: options.iter().map(|option| {
                UINode {
                    name: "radio_option".to_string(),
                    children: vec![],
                    props: HashMap::from([
                        ("value".to_string(), option.value.clone()),
                        ("label".to_string(), option.label.clone()),
                    ]),
                }
            }).collect(),
            props
        }
    }
}
