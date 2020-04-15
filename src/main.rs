mod markdown;
use markdown::Markdown;
mod config;
use config::Config;

use std::{fs, path::{Path, PathBuf}};
fn target_file_name<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut target = PathBuf::new();
    target.push("./public");
    target.push(
        path.as_ref().to_owned()
            .strip_prefix("./content").unwrap()
    );
    target.set_extension("html");
    target
}

fn convert_md_to_html<P: AsRef<Path>>(path: P, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if path.as_ref().is_dir() {
        for entry in fs::read_dir(path)?.filter_map(Result::ok) {
            convert_md_to_html(&entry.path(), config)?
        }
    } else if path.as_ref().is_file() {
        if let Some(ext) = path.as_ref().extension() {
            if ext == "md" {
                eprintln!("{:?}", path.as_ref());
                let target = target_file_name(&path);
                let markdown = Markdown::from(path).unwrap();
                fs::write(target, &markdown.to_html(config))?;
            }
        }
    }
    
    Ok(())
}


fn main() {
    let config = Config::load().unwrap();
    convert_md_to_html("./content", &config).unwrap();
}
