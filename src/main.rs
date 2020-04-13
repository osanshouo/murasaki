mod markdown;
use markdown::Markdown;

fn main() {
    let target = std::env::args().nth(1).unwrap();
    let markdown = Markdown::from(&target).unwrap();
    println!("{:?}", markdown);
}
