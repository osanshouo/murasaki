use std::fmt;
use latex2mathml::{latex_to_mathml, DisplayStyle};

#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Emphasis(String),
    Strong(String),
    Hyperlink(String, String),
    LaTeX(DisplayStyle, String),
    Paragraph(Vec<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(text) => write!(f, "{}", text),
            Node::Emphasis(text) => write!(f, "<em>{}</em>", text),
            Node::Strong(text) => write!(f, "<strong>{}</strong>", text),
            Node::Hyperlink(text, url) => write!(f, r#"<a href="{}">{}</a>"#, url, text),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let problems = vec![
            (Node::Emphasis("italic".to_owned()), "<em>italic</em>"),
            (Node::Strong("bold".to_owned()), "<strong>bold</strong>"),
        ];
        for (problem, answer) in problems.iter() {
            assert_eq!(&format!("{}", problem), answer);
        }
    }
}
