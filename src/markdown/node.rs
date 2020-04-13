use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "", )
    }
}
