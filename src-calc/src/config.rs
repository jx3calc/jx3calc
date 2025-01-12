use once_cell::sync::Lazy;
use pak;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

pub struct Config {
    value: i32,
}

impl Config {
    pub fn try_init() {
        let _ = CONFIG.value;
    }
    fn new() -> Self {
        pak::init("./cache");
        Config { value: 42 }
    }
}
