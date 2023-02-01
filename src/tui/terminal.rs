use std::io;

use super::backend::Backend;

pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
}

impl<B> Terminal<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        Ok(Terminal { backend })
    }
}
