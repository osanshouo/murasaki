pub mod date;
pub mod node;

#[derive(Debug, Clone)]
pub struct Markdown {
    path: std::path::PathBuf,
    date: date::Date,
    content: node::Node,
}
