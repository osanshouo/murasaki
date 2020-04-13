use crate::markdown::{date::Date, node::Node};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Parser { 
            input,
            chars: input.chars(), 
            cur:  '\u{0}',
            peek: '\u{0}',
        };
        parser.read_char();
        parser.read_char();
        parser
    }

    fn read_char(&mut self) {
        self.cur = self.peek;
        self.peek = self.chars.next().unwrap_or('\u{0}');
    }

    pub fn parse(&mut self) -> Result<(String, Date, Vec<String>, Vec<Node>), Box<dyn std::error::Error>> {
        let (title, date, tags) = self.parse_header()?;
        let content = self.parse_content()?;

        Ok((title, date, tags, content))
    }

    /// ヘッダー情報をパースする.
    fn parse_header(&mut self) -> Result<(String, Date, Vec<String>), Box<dyn std::error::Error>> {
        if self.input.len() < 3 { return Ok(("".to_owned(), Date(1970, 1, 1), vec![])); }
        if &self.input[..3] == "+++" {
            self.skip_line();
            while !(self.cur == '+' && self.peek == '+') { self.skip_line(); }
            self.skip_line();
            
            let mut lines = self.input.lines().skip(1);

            let title = lines.next().map_or("".to_owned(), |line| {
                line.trim().chars()
                    .skip_while(|x| x != &'\"')
                    .skip(1)
                    .take_while(|x| x != &'\"')
                    .collect::<String>()
            });
            let date = lines.next().map_or("".to_owned(), |line| {
                line.trim().chars()
                    .skip_while(|x| x != &'=')
                    .skip(1)
                    .filter(|x| !x.is_ascii_whitespace())
                    .collect::<String>()
            }).parse::<Date>().unwrap_or(Date(1970, 1, 1));
            let tags = {
                let line = lines.next().unwrap_or("+++").trim();
                if line.len() < 4 { Vec::new() } else
                if &line[..4] != "tags" { Vec::new() } else {
                    line.split(',')
                        .map(|element| element.chars()
                            .skip_while(|x| x != &'\"')
                            .skip(1)
                            .take_while(|x| x != &'\"')
                            .collect()
                        )
                        .collect()
                }
            };
            Ok((title, date, tags))
        } else {
            Ok(("".to_owned(), Date(1970, 1, 1), vec![]))
        }
    }

    fn skip_line(&mut self) {
        while self.cur != '\n' { self.read_char(); }
        self.read_char();
    }

    fn parse_content(&mut self) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        let mut nodes = Vec::new();
        while self.cur == '\u{0}' {
            nodes.push(self.parse_paragraph()?);
        }
        Ok(nodes)
    }

    fn parse_paragraph(&mut self) -> Result<Node, Box<dyn std::error::Error>> {
        let mut nodes = Vec::new();
        let node = match self.cur {
            '_' => match self.peek {
                '_' => unimplemented!(),
                _ => { 
                    self.read_char();
                    let text = self.parse_text_until("_")?;
                    Node::Emphasis(text)
                },
            },
            _ => unimplemented!(),
        };
        nodes.push(node);

        Ok(Node::Paragraph(nodes))
    }

    fn parse_text_until(&mut self, end: &str) -> Result<String, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"+++
title = "Sample Text"
date = 2020-04-01
+++
Hello, world!"#;

    const INPUT2: &str = r#"+++
title = "Sample Text 2"
date = 2020-04-01
tags = ["Rust", "日本語"]
+++
Hello, world!"#;

    #[test]
    fn header() {
        let mut parser = Parser::new(INPUT1);
        let (title, date, tags) = parser.parse_header().unwrap();
        assert_eq!(title, "Sample Text");
        assert_eq!(date,  Date(2020, 4, 1));
        assert_eq!(tags,  Vec::<String>::new());
        assert_eq!(parser.cur, 'H');

        let mut parser = Parser::new(INPUT2);
        let (title, date, tags) = parser.parse_header().unwrap();
        assert_eq!(title, "Sample Text 2");
        assert_eq!(date,  Date(2020, 4, 1));
        assert_eq!(tags,  vec!["Rust".to_owned(), "日本語".to_owned()]);
        assert_eq!(parser.cur, 'H');
    }
}
