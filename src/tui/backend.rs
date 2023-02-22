use std::io;

use crate::view::graphics::Rect;

pub mod crossterm;

pub trait Backend {
    fn size(&self) -> Result<Rect, io::Error>;
}
