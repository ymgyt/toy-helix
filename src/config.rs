use crate::view;

pub struct Config {
    pub editor: view::editor::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: view::editor::Config::default(),
        }
    }
}
