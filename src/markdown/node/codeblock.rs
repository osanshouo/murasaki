use syntect::{
    parsing::SyntaxSet,
    html::ClassedHTMLGenerator,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Cs,
    Css,
    Html,
    Javascript,
    Python,
    Rust,
    ShellScript,
    None,
}
impl Language {
    fn extension(&self) -> &'static str {
        match self {
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Cs => "cs",
            Language::Css => "css",
            Language::Html => "html",
            Language::Javascript => "js",
            Language::Python => "py",
            Language::Rust => "rs",
            Language::ShellScript => "sh",
            Language::None => "",
        }
    }
}

pub fn parse(text: String) -> (Language, Option<String>, String) {
    let mut lines = text.lines();

    let mut line = lines.next().unwrap()[3..].split(':');
    let lang = match line.next() {
        Some("bash") => Language::ShellScript,
        Some("c") => Language::C,
        Some("cpp") => Language::Cpp,
        Some("c++") => Language::Cpp,
        Some("cs") => Language::Cs,
        Some("css") => Language::Css,
        Some("html") => Language::Html,
        Some("javascript") => Language::Javascript,
        Some("python") => Language::Python,
        Some("rust") => Language::Rust,
        Some("shell") => Language::ShellScript,
        Some("shell-session") => Language::ShellScript,
        Some("shell_session") => Language::ShellScript,
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
    
    let content = match lang {
        Language::None => content,
        lang => {
            let syntax_set = SyntaxSet::load_defaults_newlines();
            let syntax = syntax_set.find_syntax_by_extension(lang.extension()).unwrap();
            let mut html_generator = ClassedHTMLGenerator::new(&syntax, &syntax_set);
            for line in content.lines() {
                html_generator.parse_html_for_line(&line);
            }
            html_generator.finalize()
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
        // assert_eq!(&content, r#"console.log("Hello, world!");"#);
    }
}
