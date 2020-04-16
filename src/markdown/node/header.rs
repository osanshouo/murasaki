use super::{Node, inline};

pub fn parse(mut text: String) -> Node {
    if &text.as_bytes()[..2] == b"# " {
        Node::Header(1, text.split_off(2))
    } else if &text.as_bytes()[..3] == b"## " {
        Node::Header(2, text.split_off(3))
    } else if &text.as_bytes()[..4] == b"### " {
        Node::Header(3, text.split_off(4))
    } else if &text.as_bytes()[..5] == b"#### " {
        Node::Header(4, text.split_off(5))
    } else if &text.as_bytes()[..6] == b"##### " {
        Node::Header(5, text.split_off(6))
    } else {
        Node::Div(inline::parse(&text))
    }
}
