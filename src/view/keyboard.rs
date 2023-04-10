use bitflags::bitflags;

bitflags! {
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

impl From<crossterm::event::KeyModifiers> for KeyModifiers {
    fn from(val: crossterm::event::KeyModifiers) -> Self {
        use crossterm::event::KeyModifiers as CKeyModifiers;

        let mut result = KeyModifiers::NONE;

        if val.contains(CKeyModifiers::SHIFT) {
            result.insert(KeyModifiers::SHIFT);
        }
        if val.contains(CKeyModifiers::CONTROL) {
            result.insert(KeyModifiers::CONTROL);
        }
        if val.contains(CKeyModifiers::ALT) {
            result.insert(KeyModifiers::ALT);
        }

        result
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// CapsLock key.
    CapsLock,
    /// ScrollLock key.
    ScrollLock,
    /// NumLock key.
    NumLock,
    /// PrintScreen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,
    /// KeypadBegin key.
    KeypadBegin,
    /// A media key.
    // Media(MediaKeyCode),
    /// A modifier key.
    Modifier(ModifierKeyCode),
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModifierKeyCode {
    /// Left Shift key.
    LeftShift,
    /// Left Control key.
    LeftControl,
    /// Left Alt key.
    LeftAlt,
    /// Left Super key.
    LeftSuper,
    /// Left Hyper key.
    LeftHyper,
    /// Left Meta key.
    LeftMeta,
    /// Right Shift key.
    RightShift,
    /// Right Control key.
    RightControl,
    /// Right Alt key.
    RightAlt,
    /// Right Super key.
    RightSuper,
    /// Right Hyper key.
    RightHyper,
    /// Right Meta key.
    RightMeta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

impl From<crossterm::event::ModifierKeyCode> for ModifierKeyCode {
    fn from(val: crossterm::event::ModifierKeyCode) -> Self {
        use crossterm::event::ModifierKeyCode as CModifierKeyCode;

        match val {
            CModifierKeyCode::LeftShift => ModifierKeyCode::LeftShift,
            CModifierKeyCode::LeftControl => ModifierKeyCode::LeftControl,
            CModifierKeyCode::LeftAlt => ModifierKeyCode::LeftAlt,
            CModifierKeyCode::LeftSuper => ModifierKeyCode::LeftSuper,
            CModifierKeyCode::LeftHyper => ModifierKeyCode::LeftHyper,
            CModifierKeyCode::LeftMeta => ModifierKeyCode::LeftMeta,
            CModifierKeyCode::RightShift => ModifierKeyCode::RightShift,
            CModifierKeyCode::RightControl => ModifierKeyCode::RightControl,
            CModifierKeyCode::RightAlt => ModifierKeyCode::RightAlt,
            CModifierKeyCode::RightSuper => ModifierKeyCode::RightSuper,
            CModifierKeyCode::RightHyper => ModifierKeyCode::RightHyper,
            CModifierKeyCode::RightMeta => ModifierKeyCode::RightMeta,
            CModifierKeyCode::IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            CModifierKeyCode::IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift,
        }
    }
}

impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(val: crossterm::event::KeyCode) -> Self {
        use crossterm::event::KeyCode as CKeyCode;

        match val {
            CKeyCode::Backspace => KeyCode::Backspace,
            CKeyCode::Enter => KeyCode::Enter,
            CKeyCode::Left => KeyCode::Left,
            CKeyCode::Right => KeyCode::Right,
            CKeyCode::Up => KeyCode::Up,
            CKeyCode::Down => KeyCode::Down,
            CKeyCode::Home => KeyCode::Home,
            CKeyCode::End => KeyCode::End,
            CKeyCode::PageUp => KeyCode::PageUp,
            CKeyCode::PageDown => KeyCode::PageDown,
            CKeyCode::Tab => KeyCode::Tab,
            CKeyCode::BackTab => unreachable!("BackTab should have been handled on KeyEvent level"),
            CKeyCode::Delete => KeyCode::Delete,
            CKeyCode::Insert => KeyCode::Insert,
            CKeyCode::F(f_number) => KeyCode::F(f_number),
            CKeyCode::Char(character) => KeyCode::Char(character),
            CKeyCode::Null => KeyCode::Null,
            CKeyCode::Esc => KeyCode::Esc,
            CKeyCode::CapsLock => KeyCode::CapsLock,
            CKeyCode::ScrollLock => KeyCode::ScrollLock,
            CKeyCode::NumLock => KeyCode::NumLock,
            CKeyCode::PrintScreen => KeyCode::PrintScreen,
            CKeyCode::Pause => KeyCode::Pause,
            CKeyCode::Menu => KeyCode::Menu,
            CKeyCode::KeypadBegin => KeyCode::KeypadBegin,
            CKeyCode::Media(media_key_code) => {
                todo!()
                // KeyCode::Media(media_key_code.into()),
            }
            CKeyCode::Modifier(modifier_key_code) => KeyCode::Modifier(modifier_key_code.into()),
        }
    }
}
