use std::io::{self, Write};

use crossterm::{
    queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor, SetForegroundColor, SetUnderlineColor,
    },
    terminal,
};

use crate::{
    tui::buffer::Cell,
    view::graphics::{Color, Modifier, Rect, UnderlineStyle},
};

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

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut underline_color = Color::Reset;
        let mut underline_style = UnderlineStyle::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            /*
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                map_error(queue!(self.buffer, MoveTo(x, y)))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut self.buffer)?;
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                let color = CColor::from(cell.fg);
                map_error(queue!(self.buffer, SetForegroundColor(color)))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = CColor::from(cell.bg);
                map_error(queue!(self.buffer, SetBackgroundColor(color)))?;
                bg = cell.bg;
            }

            let mut new_underline_style = cell.underline_style;
            if self.capabilities.has_extended_underlines {
                if cell.underline_color != underline_color {
                    let color = CColor::from(cell.underline_color);
                    map_error(queue!(self.buffer, SetUnderlineColor(color)))?;
                    underline_color = cell.underline_color;
                }
            } else {
                match new_underline_style {
                    UnderlineStyle::Reset | UnderlineStyle::Line => (),
                    _ => new_underline_style = UnderlineStyle::Line,
                }
            }

            if new_underline_style != underline_style {
                let attr = CAttribute::from(new_underline_style);
                map_error(queue!(self.buffer, SetAttribute(attr)))?;
                underline_style = new_underline_style;
            }
            */

            map_error(queue!(self.buffer, Print(&cell.symbol)))?;
        }

        map_error(queue!(
            self.buffer,
            SetUnderlineColor(CColor::Reset),
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        ))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffer.flush()
    }
}

fn map_error(error: crossterm::Result<()>) -> io::Result<()> {
    error.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}
