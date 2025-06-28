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
