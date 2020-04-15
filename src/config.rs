#[derive(Debug, Clone)]
pub struct Config {
    pub site_name: String,
    pub language: String,
    pub root: String,
    pub author: String,
    pub year: String,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
