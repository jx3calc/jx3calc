use once_cell::sync::Lazy;
use pak::init;

static CONFIG: Lazy<Config> = Lazy::new(Config::new);

pub(crate) struct Config {
    value: i32,
}

impl Config {
    pub(crate) fn try_init() {
        let _ = CONFIG.value;
    }
    fn new() -> Self {
        init("./cache");
        Config { value: 42 }
    }
}
