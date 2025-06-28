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

#[cfg(test)]
mod test {
    use fragment;

    use super::*;

    #[test]
    fn elements_macros() {
        let expected = UINode {
            name: "fragment".to_string(),
            props: HashMap::new(),
            children: vec![
                UINode {
                    name: "row".to_string(),
                    props: HashMap::new(),
                    children: vec![UINode {
                        name: "text".to_string(),
                        props: HashMap::from([
                            ("size".to_string(), "large".to_string()),
                            ("weight".to_string(), "bold".to_string()),
                            ("text".to_string(), "Теория графов".to_string()),
                        ]),
                        children: vec![],
                    }],
                },
                UINode {
                    name: "row".to_string(),
                    props: HashMap::new(),
                    children: vec![UINode {
                        name: "text".to_string(),
                        props: HashMap::from([
                            ("size".to_string(), "large".to_string()),
                            ("weight".to_string(), "medium".to_string()),
                            ("text".to_string(), "Что такое граф?".to_string()),
                        ]),
                        children: vec![],
                    }],
                },
            ],
        };

        let actual = fragment!(&[
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
        ]);

        assert_eq!(expected, actual);
    }
}
