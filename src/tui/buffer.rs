use crate::view::graphics::{Color, Modifier, Rect, Style, UnderlineStyle};

/// A buffer cell
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub symol: String,
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub underline_style: UnderlineStyle,
    pub modifier: Modifier,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            symol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            underline_color: Color::Reset,
            underline_style: UnderlineStyle::Reset,
            modifier: Modifier::empty(),
        }
    }
}

impl Cell {
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
