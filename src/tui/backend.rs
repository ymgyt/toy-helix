use std::io;

use crate::view::graphics::Rect;

use super::buffer::Cell;

pub mod crossterm;

pub trait Backend {
    fn size(&self) -> Result<Rect, io::Error>;
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<(), io::Error>;
    fn flush(&mut self) -> Result<(), io::Error>;
}
