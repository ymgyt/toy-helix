use anyhow::{anyhow, Error};

use crate::view::keyboard::{KeyCode, KeyModifiers, ModifierKeyCode};

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum Event {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    // Mouse(MouseEvent),
    Paste(String),
    Resize(u16, u16),
    IdleTimeout,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

pub(crate) mod keys {
    pub(crate) const BACKSPACE: &str = "backspace";
    pub(crate) const ENTER: &str = "ret";
    pub(crate) const LEFT: &str = "left";
    pub(crate) const RIGHT: &str = "right";
    pub(crate) const UP: &str = "up";
    pub(crate) const DOWN: &str = "down";
    pub(crate) const HOME: &str = "home";
    pub(crate) const END: &str = "end";
    pub(crate) const PAGEUP: &str = "pageup";
    pub(crate) const PAGEDOWN: &str = "pagedown";
    pub(crate) const TAB: &str = "tab";
    pub(crate) const DELETE: &str = "del";
    pub(crate) const INSERT: &str = "ins";
    pub(crate) const NULL: &str = "null";
    pub(crate) const ESC: &str = "esc";
    pub(crate) const SPACE: &str = "space";
    pub(crate) const MINUS: &str = "minus";
    pub(crate) const LESS_THAN: &str = "lt";
    pub(crate) const GREATER_THAN: &str = "gt";
    pub(crate) const CAPS_LOCK: &str = "capslock";
    pub(crate) const SCROLL_LOCK: &str = "scrolllock";
    pub(crate) const NUM_LOCK: &str = "numlock";
    pub(crate) const PRINT_SCREEN: &str = "printscreen";
    pub(crate) const PAUSE: &str = "pause";
    pub(crate) const MENU: &str = "menu";
    pub(crate) const KEYPAD_BEGIN: &str = "keypadbegin";
    pub(crate) const PLAY: &str = "play";
    pub(crate) const PAUSE_MEDIA: &str = "pausemedia";
    pub(crate) const PLAY_PAUSE: &str = "playpause";
    pub(crate) const REVERSE: &str = "reverse";
    pub(crate) const STOP: &str = "stop";
    pub(crate) const FAST_FORWARD: &str = "fastforward";
    pub(crate) const REWIND: &str = "rewind";
    pub(crate) const TRACK_NEXT: &str = "tracknext";
    pub(crate) const TRACK_PREVIOUS: &str = "trackprevious";
    pub(crate) const RECORD: &str = "record";
    pub(crate) const LOWER_VOLUME: &str = "lowervolume";
    pub(crate) const RAISE_VOLUME: &str = "raisevolume";
    pub(crate) const MUTE_VOLUME: &str = "mutevolume";
    pub(crate) const LEFT_SHIFT: &str = "leftshift";
    pub(crate) const LEFT_CONTROL: &str = "leftcontrol";
    pub(crate) const LEFT_ALT: &str = "leftalt";
    pub(crate) const LEFT_SUPER: &str = "leftsuper";
    pub(crate) const LEFT_HYPER: &str = "lefthyper";
    pub(crate) const LEFT_META: &str = "leftmeta";
    pub(crate) const RIGHT_SHIFT: &str = "rightshift";
    pub(crate) const RIGHT_CONTROL: &str = "rightcontrol";
    pub(crate) const RIGHT_ALT: &str = "rightalt";
    pub(crate) const RIGHT_SUPER: &str = "rightsuper";
    pub(crate) const RIGHT_HYPER: &str = "righthyper";
    pub(crate) const RIGHT_META: &str = "rightmeta";
    pub(crate) const ISO_LEVEL_3_SHIFT: &str = "isolevel3shift";
    pub(crate) const ISO_LEVEL_5_SHIFT: &str = "isolevel5shift";
}

impl std::str::FromStr for KeyEvent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens: Vec<_> = s.split('-').collect();
        let code = match tokens.pop().ok_or_else(|| anyhow!("Missing key code"))? {
            keys::BACKSPACE => KeyCode::Backspace,
            keys::ENTER => KeyCode::Enter,
            keys::LEFT => KeyCode::Left,
            keys::RIGHT => KeyCode::Right,
            keys::UP => KeyCode::Up,
            keys::DOWN => KeyCode::Down,
            keys::HOME => KeyCode::Home,
            keys::END => KeyCode::End,
            keys::PAGEUP => KeyCode::PageUp,
            keys::PAGEDOWN => KeyCode::PageDown,
            keys::TAB => KeyCode::Tab,
            keys::DELETE => KeyCode::Delete,
            keys::INSERT => KeyCode::Insert,
            keys::NULL => KeyCode::Null,
            keys::ESC => KeyCode::Esc,
            keys::SPACE => KeyCode::Char(' '),
            keys::MINUS => KeyCode::Char('-'),
            keys::LESS_THAN => KeyCode::Char('<'),
            keys::GREATER_THAN => KeyCode::Char('>'),
            keys::CAPS_LOCK => KeyCode::CapsLock,
            keys::SCROLL_LOCK => KeyCode::ScrollLock,
            keys::NUM_LOCK => KeyCode::NumLock,
            keys::PRINT_SCREEN => KeyCode::PrintScreen,
            keys::PAUSE => KeyCode::Pause,
            keys::MENU => KeyCode::Menu,
            keys::KEYPAD_BEGIN => KeyCode::KeypadBegin,
            keys::LEFT_SHIFT => KeyCode::Modifier(ModifierKeyCode::LeftShift),
            keys::LEFT_CONTROL => KeyCode::Modifier(ModifierKeyCode::LeftControl),
            keys::LEFT_ALT => KeyCode::Modifier(ModifierKeyCode::LeftAlt),
            keys::LEFT_SUPER => KeyCode::Modifier(ModifierKeyCode::LeftSuper),
            keys::LEFT_HYPER => KeyCode::Modifier(ModifierKeyCode::LeftHyper),
            keys::LEFT_META => KeyCode::Modifier(ModifierKeyCode::LeftMeta),
            keys::RIGHT_SHIFT => KeyCode::Modifier(ModifierKeyCode::RightShift),
            keys::RIGHT_CONTROL => KeyCode::Modifier(ModifierKeyCode::RightControl),
            keys::RIGHT_ALT => KeyCode::Modifier(ModifierKeyCode::RightAlt),
            keys::RIGHT_SUPER => KeyCode::Modifier(ModifierKeyCode::RightSuper),
            keys::RIGHT_HYPER => KeyCode::Modifier(ModifierKeyCode::RightHyper),
            keys::RIGHT_META => KeyCode::Modifier(ModifierKeyCode::RightMeta),
            keys::ISO_LEVEL_3_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift),
            keys::ISO_LEVEL_5_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift),
            single if single.chars().count() == 1 => KeyCode::Char(single.chars().next().unwrap()),
            function if function.len() > 1 && function.starts_with('F') => {
                let function: String = function.chars().skip(1).collect();
                let function = str::parse::<u8>(&function)?;
                (function > 0 && function < 13)
                    .then(|| KeyCode::F(function))
                    .ok_or_else(|| anyhow!("Invalid function key '{}'", function))?
            }
            invalid => return Err(anyhow!("Invalid key code '{}'", invalid)),
        };

        let mut modifiers = KeyModifiers::empty();
        for token in tokens {
            let flag = match token {
                "S" => KeyModifiers::SHIFT,
                "A" => KeyModifiers::ALT,
                "C" => KeyModifiers::CONTROL,
                _ => return Err(anyhow!("Invalid key modifier '{}-'", token)),
            };

            if modifiers.contains(flag) {
                return Err(anyhow!("Repeated key modifier '{}-'", token));
            }
            modifiers.insert(flag);
        }

        Ok(KeyEvent { code, modifiers })
    }
}

impl From<crossterm::event::Event> for Event {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(key) => Self::Key(key.into()),
            crossterm::event::Event::Mouse(mouse) => {
                todo!()
                // Self::Mouse(mouse.into()),
            }
            crossterm::event::Event::Resize(w, h) => Self::Resize(w, h),
            crossterm::event::Event::FocusGained => Self::FocusGained,
            crossterm::event::Event::FocusLost => Self::FocusLost,
            crossterm::event::Event::Paste(s) => Self::Paste(s),
        }
    }
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(crossterm::event::KeyEvent { code, modifiers, .. }: crossterm::event::KeyEvent) -> Self {
        if code == crossterm::event::KeyCode::BackTab {
            // special case for BackTab -> Shift-Tab
            let mut modifiers: KeyModifiers = modifiers.into();
            modifiers.insert(KeyModifiers::SHIFT);
            Self {
                code: KeyCode::Tab,
                modifiers,
            }
        } else {
            Self {
                code: code.into(),
                modifiers: modifiers.into(),
            }
        }
    }
}
