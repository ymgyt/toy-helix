use std::num::NonZeroUsize;

pub mod document;
pub mod editor;
pub mod graphics;
pub mod input;
pub mod keyboard;
pub mod theme;
pub mod tree;
pub mod view;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentId(NonZeroUsize);

impl Default for DocumentId {
    fn default() -> DocumentId {
        DocumentId(unsafe { NonZeroUsize::new_unchecked(1) })
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

slotmap::new_key_type! {
    pub struct ViewId;
}
