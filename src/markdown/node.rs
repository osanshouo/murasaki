use std::fmt;
use latex2mathml::{latex_to_mathml, DisplayStyle};

pub mod inline;
pub mod codeblock;
use codeblock::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(String),
    Emphasis(String),
    Strong(String),
    // Hyperlink(String, String),
    LaTeX(DisplayStyle, String),
    // Paragraph(Vec<Node>),
    CodeBlock(Language, Option<String>, String),
    Div(Vec<Node>),
}

impl Node {
    /// ブロックの種類に応じて処理を分岐する.
    /// - テキスト
    /// - コードブロック
    /// - ヘッダー (未実装)
    /// - リスト (未実装)
    pub fn parse(text: String) -> Node {
        if text.len() < 3 {
            Node::Div(vec![Node::Text(text)])
        } else if &text.as_bytes()[..3] == b"```" {
            let (language, filename, content) = codeblock::parse(text);
            Node::CodeBlock(language, filename, content)
        } else {
            Node::Div(inline::parse(&text))
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(text) => write!(f, "{}", text),
            Node::Emphasis(text) => write!(f, "<em>{}</em>", text),
            Node::Strong(text) => write!(f, "<strong>{}</strong>", text),
            // Node::Hyperlink(text, url) => write!(f, r#"<a href="{}">{}</a>"#, url, text),
            Node::LaTeX(style, text) => match latex_to_mathml(text, style.clone()) {
                Ok(mathml) => write!(f, "{}", mathml),
                Err(_)     => write!(f, "${}$", text),
            },
            // Node::Paragraph(vec) => {
            //     write!(f, r#"<div class="paragraph">{}</div>"#,
            //         vec.iter().map(|node| format!("{}", node)).collect::<String>()
            //     )
            // },
            Node::CodeBlock(_, filename, content) => {
                let filename = match filename {
                    Some(filename) => format!(r#"<div class="filename">{}</div>"#, filename),
                    None => "".to_string(),
                };
                write!(f, r#"<div class="code-frame">{}<pre>{}</pre></div>"#, filename, content)
            },
            Node::Div(vec) => write!(f, r#"<div class=>{}</div>"#, 
                vec.iter().map(|node| format!("{}", node)).collect::<String>()
            ),
            // _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let problems: Vec<(Node, &str)> = vec![
            // (Node::Emphasis("italic".to_owned()), "<em>italic</em>"),
            // (Node::Strong("bold".to_owned()), "<strong>bold</strong>"),
        ];
        for (problem, answer) in problems.iter() {
            assert_eq!(&format!("{}", problem), answer);
        }
    }
}
