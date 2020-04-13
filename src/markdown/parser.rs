use crate::markdown::{date::Date, node::Node};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    input: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Parser { 
            input: input.chars(), 
            cur:  '\u{0}',
            peek: '\u{0}',
        };
        parser.read_char();
        parser.read_char();
        parser
    }

    fn read_char(&mut self) {
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
    }

    pub fn parse(&mut self) -> Result<(String, Date, Vec<String>, Vec<Node>), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
