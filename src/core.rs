pub use encoding_rs as encoding;

pub mod chars;
pub mod doc_formatter;
pub mod graphemes;
pub mod line_ending;
pub mod macros;
pub mod movement;
pub mod path;
pub mod position;
pub mod selection;
pub mod syntax;
pub mod text_annotations;

pub mod unicode {
    pub use unicode_general_category as category;
    pub use unicode_segmentation as sementation;
    pub use unicode_width as width;
}

pub use ropey::{str_utils, Rope, RopeBuilder, RopeSlice};

pub use smartstring::SmartString;

pub use line_ending::LineEnding;

pub use selection::{Range, Selection};

pub type Tendril = SmartString<smartstring::LazyCompact>;
