use std::collections::HashMap;

use crate::{
    term::keymap::{self, Keymap},
    view::{self, document::Mode},
};

pub struct Config {
    pub keys: HashMap<Mode, Keymap>,
    pub editor: view::editor::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keys: keymap::default(),
            editor: view::editor::Config::default(),
        }
    }
}
