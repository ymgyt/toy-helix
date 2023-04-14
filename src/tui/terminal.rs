use std::io;

use crate::view::graphics::{CursorKind, Rect};

use super::{backend::Backend, buffer::Buffer};

#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    area: Rect,
}

impl Viewport {
    pub fn fixed(area: Rect) -> Viewport {
        Viewport { area }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerminalOptions {
    pub viewport: Viewport,
}

pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    /// Holds the results of the current adn previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    pub current: usize,
}

impl<B> Terminal<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let size = backend.size()?;

        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport { area: size },
            },
        )
    }

    pub fn with_options(backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(options.viewport.area), Buffer::empty(options.viewport.area)],
            current: 0,
        })
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn autoresize(&mut self) -> io::Result<Rect> {
        let size = self.size()?;
        // TODO: compare self.viewport.area
        Ok(size)
    }

    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        self.backend.draw(updates.into_iter())
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw(&mut self, cursor_position: Option<(u16, u16)>, cursor_kind: CursorKind) -> io::Result<()> {
        self.flush()?;

        // TODO: handle cursor

        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        self.backend.flush()?;
        Ok(())
    }
}
