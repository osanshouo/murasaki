use latex2mathml::DisplayStyle;
use super::Node;

const DELIMITER: [char; 5] = [ '*', '_', '[', '!', '$'];

#[derive(Debug, Clone)]
struct InlineParser<'a> {
    chars: std::str::Chars<'a>,
    cur: char,
    peek: char,
}
impl<'a> InlineParser<'a> {
    fn new(text: &'a str) -> InlineParser<'a> {
        let mut chars = text.chars();
        let cur = chars.next().unwrap_or('\u{0}');
        let peek = chars.next().unwrap_or('\u{0}');
        InlineParser { chars, cur, peek }
    }

    fn next(&mut self) {
        self.cur = self.peek;
        self.peek = self.chars.next().unwrap_or('\u{0}');
    }

    fn read_text(&mut self) -> String {
        let mut buf = String::new();
        while !DELIMITER.contains(&self.cur) && self.cur !='\u{0}' {
            buf.push(self.cur);
            self.next();
        }
        buf
    }

    fn read_until1(&mut self, end: char) -> String {
        let mut buf = String::new();
        while self.cur != end && self.cur !='\u{0}' {
            buf.push(self.cur);
            self.next();
        }
        self.next();
        buf
    }

    fn read_until2(&mut self, end: char) -> String {
        let mut buf = String::new();
        while !(self.cur == end && self.peek == end) && self.cur !='\u{0}' {
            buf.push(self.cur);
            self.next();
        }
        for _ in 0..2 { self.next(); }
        buf
    }
}

pub fn parse(text: &str) -> Vec<Node> {
    let mut div = Vec::new();
    let mut parser = InlineParser::new(text);

    let inline = parser.read_text();
    div.push(Node::Text(inline));

    loop {
        match parser.cur {
            '\u{0}' => break,
            '_' => if parser.peek == '_' {
                for _ in 0..2 { parser.next(); }
                let inline = parser.read_until2('_');
                div.push(Node::Strong(inline));
            } else {
                parser.next();
                let inline = parser.read_until1('_');
                div.push(Node::Emphasis(inline));
            },
            '*' => if parser.peek == '*' {
                for _ in 0..2 { parser.next(); }
                let inline = parser.read_until2('*');
                div.push(Node::Strong(inline));
            } else {
                parser.next();
                let inline = parser.read_until1('*');
                div.push(Node::Emphasis(inline));
            },
            '$' => if parser.peek == '$' {
                for _ in 0..2 { parser.next(); }
                let inline = parser.read_until2('$');
                div.push(Node::LaTeX(DisplayStyle::Block, inline));
            } else {
                parser.next();
                let inline = parser.read_until1('$');
                div.push(Node::LaTeX(DisplayStyle::Inline, inline));
            },
            '[' => unimplemented!(),
            '!' => unimplemented!(),
            _ => {
                let inline = parser.read_text();
                div.push(Node::Text(inline));
            },
        }
    }
    
    div
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_parse() {
        let problems = vec![
            ("_italic_", vec![Node::Emphasis("italic".to_owned())]),
            ("__bold__", vec![Node::Strong("bold".to_owned())]),
            ("This is an _italic_ word.", vec![
                Node::Text("This is an ".to_owned()),
                Node::Emphasis("italic".to_owned()),
                Node::Text(" word.".to_owned()),
            ]),
        ];

        for (problem, answer) in problems.iter() {
            assert_eq!(&parse(problem), answer);
        }
    }
}