pub mod date;
pub mod node;
pub mod parser;
use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub struct Markdown {
    path: PathBuf,
    date: Option<date::Date>,
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

    fn tags_to_html(&self) -> String {
        self.tags.iter()
            .map(|tag| {
                let mut slug = tag.clone();
                slug.make_ascii_lowercase();
                format!(r#"<div class="tag"><a href="{{{{ROOT}}}}/tags/{}">{}</a></div>"#, slug, tag)
            })
            .collect()
    }

    fn date_to_html(&self) -> String {
        match self.date {
            Some(date) => format!("<time datetime={0}>{0}</time>", date),
            None => String::new(),
        }
    }

    pub fn to_html(&self, config: &crate::config::Config) -> String {
        CONTENT_TEMPLATE
            .replace("{{SITE NAME}}", &config.site_name)
            .replace("{{LANG}}", &config.language)
            .replace("{{TITLE}}", &self.title)
            .replace("{{AUTHOR}}", &config.author)
            .replace("{{YEAR}}", &config.year)
            .replace("{{TAGS}}", &self.tags_to_html())
            .replace("{{DATE}}", &self.date_to_html())
            .replace("{{CONTENT}}", "")
            .replace("{{ROOT}}", &config.root)
    }
}

const CONTENT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="{{LANG}}">

<head>
  <meta charset="utf-8">
  <title>{{TITLE}} - {{SITE NAME}}</title>
  <link rel="stylesheet" href="/article-page.css">
</head>

<body>
  <header>
    <div class="header-container"><a href="{{ROOT}}">{{SITE NAME}}</a></div>
  </header>

  <main class="main">
<section class="article">
  <header>
    <h1 class="title">{{TITLE}}</h1>
    <div class="tags-container">
        {{TAGS}}
        <div class="date">{{DATE}}</div>
    </div>
  </header>

  CONTENT
</section>
  </main>

  <footer>
    <div class="footer-container">&copy;{{YEAR}} {{AUTHOR}}</div>
  </footer>
</html>
"#;
