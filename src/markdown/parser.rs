use crate::markdown::{date::Date, node::Node};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { lines: input.lines() }
    }

    pub fn parse(&mut self) -> Result<(String, Option<Date>, Vec<String>, Vec<Node>), Box<dyn std::error::Error>> {
        let (title, date, tags) = self.parse_header()?;
        let blocks = self.divide_into_blocks()?;
        let blocks = blocks.into_iter()
            .map(|block| Node::parse(block))
            .collect();
        
        Ok((title, date, tags, blocks))
    }

    /// ヘッダー情報をパースする.
    /// 
    /// この関数を終了した時点で `self.lines` は "---" までを消費している.
    fn parse_header(&mut self) -> Result<(String, Option<Date>, Vec<String>), Box<dyn std::error::Error>> {
        assert_eq!(self.lines.next(), Some("---"));
        let mut line;

        line = self.lines.next().unwrap();
        assert_eq!(&line[..7], "title: ");
        let title = line[7..].trim().to_owned();
        line = self.lines.next().unwrap();

        let date = if &line[..6.min(line.len())] == "date: " { 
            let date = line[6..].trim().parse().unwrap();
            line = self.lines.next().unwrap();
            Some(date)
        } else { None };

        let tags = if &line[..6.min(line.len())] == "tags: " { 
            let tags = line[6..].trim().split(' ').map(|tag| tag.to_owned()).collect::<Vec<_>>();
            line = self.lines.next().unwrap();
            tags
        } else { vec![] };

        while &line[..3.min(line.len())] != "---" {
            line = self.lines.next().unwrap();
        }

        Ok((title, date, tags))
    }

    fn divide_into_blocks(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut document = Vec::new();
        let mut div = Vec::new();
        while let Some(line) = self.lines.next() {
            if line != "" {
                div.push(line);
            } else {
                let block = div.join("\n");
                if block.len() > 0 { document.push(block); }
                div = Vec::new();
            }
        }
        let block = div.join("\n");
        if block.len() > 0 { document.push(block); }
        
        Ok(document)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"---
title: Sample Text
---
Hello, world!"#;

    const INPUT2: &str = r#"---
title: Sample Text 2
date: 2020-04-01
tags: Rust 日本語
dummy: field
---
Hello, world!

```rust:src/main.rs
fn main() {
    println!("Hello, world!");
}
```



This is an example of list.

- First line.
- Second line.
"#;

    #[test]
    fn header() {
        let mut parser = Parser::new(INPUT1);
        let (title, date, tags) = parser.parse_header().unwrap();
        assert_eq!(title, "Sample Text");
        assert_eq!(date,  None);
        assert_eq!(tags,  Vec::<String>::new());
        assert_eq!(parser.lines.next(), Some("Hello, world!"));

        let mut parser = Parser::new(INPUT2);
        let (title, date, tags) = parser.parse_header().unwrap();
        assert_eq!(title, "Sample Text 2");
        assert_eq!(date,  Some(Date(2020, 4, 1)));
        assert_eq!(tags,  vec!["Rust".to_owned(), "日本語".to_owned()]);
        let content = parser.divide_into_blocks().unwrap();
        println!("{:?}", content);
        assert_eq!(content.len(), 4)
    }
}
