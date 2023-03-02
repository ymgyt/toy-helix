pub use encoding_rs as encoding;

pub mod chars;
pub mod doc_formatter;
pub mod graphemes;
pub mod macros;
pub mod path;
pub mod position;
pub mod syntax;
pub mod text_annotations;

pub use ropey::{str_utils, Rope, RopeBuilder, RopeSlice};

pub use smartstring::SmartString;

pub type Tendril = SmartString<smartstring::LazyCompact>;
