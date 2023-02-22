use std::io::{self, Write};

use crossterm::terminal;

use crate::view::graphics::Rect;

use super::Backend;

pub struct CrosstermBackend<W: Write> {
    buffer: W,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> CrosstermBackend<W> {
        CrosstermBackend { buffer }
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    fn size(&self) -> Result<crate::view::graphics::Rect, std::io::Error> {
        let (width, height) = terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Rect::new(0, 0, width, height))
    }
}
