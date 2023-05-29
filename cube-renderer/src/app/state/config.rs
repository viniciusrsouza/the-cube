#[derive(Debug)]
pub struct Config {
    pub host: String,
}

impl Config {
    pub fn new(host: String) -> Config {
        Config { host }
    }
}
