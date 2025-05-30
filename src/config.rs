pub struct Config {
    pub interval: u32,
}

impl Config {
    pub fn new() -> Self {
        Config { interval: 3000 }
    }
}
