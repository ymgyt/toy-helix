use std::str::FromStr;

use bitflags::bitflags;

/// A simple rectangle used to in the computation of the layout and to give wiwdgets an hint about the
/// area they are supposed to render to. (x, y) = (0, 0) is at the top left corner of the screen.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect { x, y, width, height }
    }

    #[inline]
    pub fn area(self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    #[inline]
    pub fn left(self) -> u16 {
        self.x
    }

    #[inline]
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    #[inline]
    pub fn top(self) -> u16 {
        self.y
    }

    #[inline]
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn clip_bottom(self, height: u16) -> Rect {
        Rect {
            height: self.height.saturating_sub(height),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    Reset,
    Line,
    Curl,
    Dotted,
    Dashed,
    DoubleLine,
}

impl FromStr for UnderlineStyle {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "line" => Ok(Self::Line),
            "curl" => Ok(Self::Curl),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "double_line" => Ok(Self::DoubleLine),
            _ => Err("Invalid underline style"),
        }
    }
}

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    /// They are bitflags so they can easily be composed.
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

impl FromStr for Modifier {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "bold" => Ok(Self::BOLD),
            "dim" => Ok(Self::DIM),
            "italic" => Ok(Self::ITALIC),
            "slow_blink" => Ok(Self::SLOW_BLINK),
            "rapid_blink" => Ok(Self::RAPID_BLINK),
            "reversed" => Ok(Self::REVERSED),
            "hidden" => Ok(Self::HIDDEN),
            "crossed_out" => Ok(Self::CROSSED_OUT),
            _ => Err("Invalid modifier"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub underline_style: Option<UnderlineStyle>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

impl Style {
    pub fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    pub fn underline_color(mut self, color: Color) -> Style {
        self.underline_color = Some(color);
        self
    }

    pub fn underline_style(mut self, style: UnderlineStyle) -> Style {
        self.underline_style = Some(style);
        self
    }

    pub fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier.remove(modifier);
        self.add_modifier.insert(modifier);
        self
    }

    pub fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier.remove(modifier);
        self.sub_modifier.insert(modifier);
        self
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: None,
            bg: None,
            underline_color: None,
            underline_style: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }
}
