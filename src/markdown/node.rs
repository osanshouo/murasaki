use std::fmt;
use latex2mathml::{latex_to_mathml, DisplayStyle};

#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Emphasis(String),
    Strong(String),
    LaTeX(DisplayStyle, String),
    Paragraph(Vec<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(text) => write!(f, "{}", text),
            Node::Emphasis(text) => write!(f, "<em>{}</em>", text),
            Node::Strong(text) => write!(f, "<strong>{}</strong>", text),
            Node::LaTeX(style, text) => match latex_to_mathml(text, style.clone()) {
                Ok(mathml) => write!(f, "{}", mathml),
                Err(_)     => write!(f, "${}$", text),
            },
            Node::Paragraph(vec) => {
                write!(f, r#"<div class="paragraph">{}</div>"#,
                    vec.iter().map(|node| format!("{}", node)).collect::<String>()
                )
            },
        }
    }
}
