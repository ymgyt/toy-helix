use ropey::RopeSlice;

use crate::core::{
    doc_formatter::TextFormat,
    graphemes::{nth_next_grapheme_boundary, nth_prev_grapheme_boundary},
    text_annotations::TextAnnotations,
    Range,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Movement {
    Extend,
    Move,
}
pub fn move_horizontally(
    slice: RopeSlice,
    range: Range,
    dir: Direction,
    count: usize,
    behaviour: Movement,
    _: &TextFormat,
    _: &mut TextAnnotations,
) -> Range {
    let pos = range.cursor(slice);

    // Compute the new position.
    let new_pos = match dir {
        Direction::Forward => nth_next_grapheme_boundary(slice, pos, count),
        Direction::Backward => nth_prev_grapheme_boundary(slice, pos, count),
    };

    range.put_cursor(slice, new_pos, behaviour == Movement::Extend)
}
