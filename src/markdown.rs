pub mod date;
pub mod node;
pub mod parser;
use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub struct Markdown {
    path: PathBuf,
    date: date::Date,
    title: String,
    tags: Vec<String>,
    content: Vec<node::Node>,
}

impl Markdown {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Markdown, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        let mut parser = parser::Parser::new(&content);
        let path = {
            let mut path = path.as_ref().to_owned();
            path.set_extension("html");
            path
        };
        let (title, date, tags, content) = parser.parse()?;
        
        Ok(Markdown { path, date, title, tags, content })
    }
}
