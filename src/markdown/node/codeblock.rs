use syntect::{
    parsing::SyntaxSet,
    highlighting::{ThemeSet},
    html::{
        highlighted_html_for_string,
    },
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    C,
    Css,
    Html,
    Javascript,
    Python,
    Rust,
    None,
}
impl Language {
    fn extension(&self) -> &'static str {
        match self {
            Language::C => "c",
            Language::Css => "css",
            Language::Html => "html",
            Language::Javascript => "js",
            Language::Python => "py",
            Language::Rust => "rs",
            Language::None => "",
            
        }
    }
}

pub fn parse(text: String) -> (Language, Option<String>, String) {
    let mut lines = text.lines();

    let mut line = lines.next().unwrap()[3..].split(':');
    let lang = match line.next() {
        Some("c") => Language::C,
        Some("css") => Language::Css,
        Some("html") => Language::Html,
        Some("javascript") => Language::Javascript,
        Some("python") => Language::Python,
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
        Language::None => content,
        lang => {
            let syntax = ps.find_syntax_by_extension(lang.extension()).unwrap();
            highlighted_html_for_string(&content, &ps, syntax, &ts.themes["base16-mocha.dark"])
        },
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
        let (lang, filename, _content) = parse(text);
        assert_eq!(lang, Language::Rust);
        assert_eq!(filename, Some("src/main.rs".to_owned()));
    }
}
