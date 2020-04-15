use syntect::{
    parsing::SyntaxSet,
    highlighting::{ThemeSet},
    html::{
        highlighted_html_for_string,
    },
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    None,
}

pub fn parse(text: String) -> (Language, Option<String>, String) {
    let mut lines = text.lines();

    let mut line = lines.next().unwrap()[3..].split(':');
    let lang = match line.next() {
        Some("rust") => Language::Rust,
        _ => Language::None,
    };
    let filename = line.next().map(|fp| fp.to_owned());

    let mut content = Vec::new();
    for line in lines {
        if line != "```" {
            content.push(line);
        }
    }
    let content = content.join("\n");
    
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let content = match lang {
        Language::Rust => {
            let syntax = ps.find_syntax_by_extension("rs").unwrap();
            highlighted_html_for_string(&content, &ps, syntax, &ts.themes["base16-mocha.dark"])
        },
        _ => content,
    };

    (lang, filename, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = r#"```
console.log("Hello, world!");
```"#.to_owned();
        let (lang, filename, content) = parse(text);
        assert_eq!(lang, Language::None);
        assert_eq!(filename, None);
        assert_eq!(&content, r#"console.log("Hello, world!");"#);

        let text = r#"```rust:src/main.rs
fn main() {
    println!("Hello, world!");
}
```"#.to_owned();
        let (lang, filename, content) = parse(text);
        assert_eq!(lang, Language::Rust);
        assert_eq!(filename, Some("src/main.rs".to_owned()));
        assert_eq!(&content, r#"fn main() {
    println!("Hello, world!");
}"#);
    }
}
