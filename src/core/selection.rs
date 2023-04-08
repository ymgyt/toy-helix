use ropey::RopeSlice;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    /// The anchor of the range: the side that doesn't move when extending.
    pub anchor: usize,
    /// The head of the range, moved when extending.
    pub head: usize,
    /// The previous visual offset (softwrapped lines and columns) from
    /// the start of the line
    pub old_visual_position: Option<(u32, u32)>,
}

impl Range {
    pub fn new(anchor: usize, head: usize) -> Self {
        Self {
            anchor,
            head,
            old_visual_position: None,
        }
    }

    pub fn grapheme_aligned(&self, _slice: RopeSlice) -> Self {
        // TODO: handle grapheme boundary
        Range {
            anchor: self.anchor,
            head: self.head,
            old_visual_position: self.old_visual_position,
        }
    }
}

/// A selection consists of one or more selection ranges.
/// invariant: A selection can never be emtpy (always contains at least primary range).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    ranges: SmallVec<[Range; 1]>,
    primary_index: usize,
}

impl Selection {
    pub fn ensure_invariants(self, text: RopeSlice) -> Self {
        // TODO: imple
        self
    }

    /// Constructs a selection holding a single range.
    pub fn single(anchor: usize, head: usize) -> Self {
        Self {
            ranges: smallvec![Range {
                anchor,
                head,
                old_visual_position: None
            }],
            primary_index: 0,
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, Range> {
        self.ranges.iter()
    }
}
