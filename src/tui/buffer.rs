use std::{cmp::min, usize::MAX};

use crate::core::unicode::width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;

use crate::view::graphics::{Color, Modifier, Rect, Style, UnderlineStyle};

/// A buffer cell
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub symbol: String,
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub underline_style: UnderlineStyle,
    pub modifier: Modifier,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            symbol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            underline_color: Color::Reset,
            underline_style: UnderlineStyle::Reset,
            modifier: Modifier::empty(),
        }
    }
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push_str(symbol);
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        if let Some(c) = style.fg {
            self.fg = c;
        }
        if let Some(c) = style.bg {
            self.bg = c;
        }
        if let Some(c) = style.underline_color {
            self.underline_color = c;
        }
        if let Some(style) = style.underline_style {
            self.underline_style = style;
        }

        self.modifier.insert(style.add_modifier);
        self.modifier.remove(style.sub_modifier);
        self
    }

    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.underline_color = Color::Reset;
        self.underline_style = UnderlineStyle::Reset;
        self.modifier = Modifier::empty();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// The area represented by this buffer
    pub area: Rect,
    /// The content of the buffer. The length of this Vec should always be equal to area.width * area.height
    pub content: Vec<Cell>,
}

impl Buffer {
    /// Returns a Buffer with all cells set to the default one
    pub fn empty(area: Rect) -> Buffer {
        let cell: Cell = Default::default();
        Buffer::filled(area, &cell)
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
    pub fn filled(area: Rect, cell: &Cell) -> Buffer {
        let size = area.area();
        let mut content = Vec::with_capacity(size);
        for _ in 0..size {
            content.push(cell.clone());
        }
        Buffer { area, content }
    }

    pub fn set_style(&mut self, area: Rect, style: Style) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                self[(x, y)].set_style(style);
            }
        }
    }

    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
        }
    }

    /// Tells wether the global (x, y) coordinates are inside the Buffers'a area.
    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x >= self.area.left() && x < self.area.right() && y >= self.area.top() && y < self.area.bottom()
    }

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            self.in_bounds(x, y),
            "Trying to access position outside the buffer: x={x}, y={y}, area={:?}",
            self.area,
        );
        ((y - self.area.y) as usize) * (self.area.width as usize) + ((x - self.area.x) as usize)
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string<S>(&mut self, x: u16, y: u16, string: S, style: Style)
    where
        S: AsRef<str>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    /// Print at most the first n characters of a string if enough space is avaialble
    /// until the end of the line
    pub fn set_stringn<S>(&mut self, x: u16, y: u16, string: S, width: usize, style: Style) -> (u16, u16)
    where
        S: AsRef<str>,
    {
        self.set_string_truncated_at_end(x, y, string.as_ref(), width, style)
    }
    /// Print at most the first `width` characters of a string if enough space is available
    /// until the end of the line.
    pub fn set_string_truncated_at_end(&mut self, x: u16, y: u16, string: &str, width: usize, style: Style) -> (u16, u16) {
        // prevent panic if out of range
        if !self.in_bounds(x, y) {
            return (x, y);
        }

        let mut index = self.index_of(x, y);
        let mut x_offset = x as usize;
        let max_x_offset = min(self.area.right() as usize, width.saturating_add(x as usize));

        for s in string.graphemes(true) {
            let width = s.width();
            if width == 0 {
                continue;
            }
            // `x_offset + width > max_offset` could be integer overflow on 32-bit machines if we
            // change dimensions to usize or u32 and someone resizes the terminal to 1x2^32.
            if width > max_x_offset.saturating_sub(x_offset) {
                break;
            }

            self.content[index].set_symbol(s);
            self.content[index].set_style(style);
            // TODO: enable
            // Reset following cells if multi-width (they would be hidden by the grapheme),
            // for i in index + 1..index + width {
            //     self.content[i].reset();
            // }
            index += width;
            x_offset += width;
        }

        (x_offset as u16, y)
    }

    pub fn diff<'a>(&self, other: &'a Buffer) -> Vec<(u16, u16, &'a Cell)> {
        let previous_buffer = &self.content;
        let next_buffer = &other.content;
        let width = self.area.width;

        let mut updates: Vec<(u16, u16, &Cell)> = vec![];
        let mut invalidated: usize = 0;
        let mut to_skip: usize = 0;
        for (i, (current, previous)) in next_buffer.iter().zip(previous_buffer.iter()).enumerate() {
            if (current != previous || invalidated > 0) && to_skip == 0 {
                let x = (i % width as usize) as u16;
                let y = (i / width as usize) as u16;
                updates.push((x, y, &next_buffer[i]));
            }

            let current_width = current.symbol.width();
            to_skip = current_width.saturating_sub(1);

            let affected_width = std::cmp::max(current_width, previous.symbol.width());
            invalidated = std::cmp::max(affected_width, invalidated).saturating_sub(1);
        }
        updates
    }
}

impl std::ops::Index<(u16, u16)> for Buffer {
    type Output = Cell;

    fn index(&self, (x, y): (u16, u16)) -> &Self::Output {
        let i = self.index_of(x, y);
        &self.content[i]
    }
}

impl std::ops::IndexMut<(u16, u16)> for Buffer {
    fn index_mut(&mut self, (x, y): (u16, u16)) -> &mut Self::Output {
        let i = self.index_of(x, y);
        &mut self.content[i]
    }
}
