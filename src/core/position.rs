use ropey::RopeSlice;

use super::{
    doc_formatter::{DocumentFormatter, TextFormat},
    text_annotations::TextAnnotations,
};

/// Represents a single point in a text buffer. Zero indexed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

/// Returns the visual offset from the start of the first visual line
/// in the block that contains anchor.
/// Text is always wrapped at blocks, they usually correspond to
/// actual line breaks but for very long lines
/// softwrapping positions are estimated with an O(1) algorithm
/// to ensure consistent performance for large lines (currently unimplemented)
///
/// Usualy you want to use `visual_offset_from_anchor` instead but this function
/// can be useful (and faster) if
/// * You already know the visual position of the block
/// * You only care about the horizontal offset (column) and not the vertical offset (row)
pub fn visual_offset_from_block(
    text: RopeSlice,
    anchor: usize,
    pos: usize,
    text_fmt: &TextFormat,
    annotations: &TextAnnotations,
) -> (Position, usize) {
    let mut last_pos = Position::default();
    let (formatter, block_start) = DocumentFormatter::new_at_prev_checkpoint(text, text_fmt, annotations, anchor);
    let mut char_pos = block_start;

    for (grapheme, vpos) in formatter {
        last_pos = vpos;
        char_pos += grapheme.doc_chars();

        if char_pos > pos {
            return (last_pos, block_start);
        }
    }

    (last_pos, block_start)
}

/// Returns the visual offset from the start of the visual line
/// that contains anchor.
pub fn visual_offset_from_anchor(
    text: RopeSlice,
    anchor: usize,
    text_fmt: &TextFormat,
    annotations: &TextAnnotations,
    max_rows: usize,
) -> Result<(Position, usize), VisualOffsetError> {
    todo!()
}
