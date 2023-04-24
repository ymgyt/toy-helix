use std::collections::HashMap;

use crate::core::macros::hashmap;
use crate::view::document::Mode;

use super::macros::keymap;
use super::Keymap;

pub fn default() -> HashMap<Mode, Keymap> {
    let normal = keymap!({ "Normal mode"
    "l" => move_char_right,
    "h" => move_char_left,
    "j" => move_visual_line_down,
    "k" => move_visual_line_up,
    "q" => _quit,
    });

    hashmap!(
        Mode::Normal => Keymap::new(normal),
    )
}
