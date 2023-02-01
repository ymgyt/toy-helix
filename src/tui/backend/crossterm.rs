use std::io::Write;

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

impl<W> Backend for CrosstermBackend<W> where W: Write {}
