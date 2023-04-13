use ropey::RopeSlice;
use smallvec::{smallvec, SmallVec};

use super::graphemes::{next_grapheme_boundary, nth_next_grapheme_boundary, prev_grapheme_boundary};

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

    pub fn point(head: usize) -> Self {
        Self::new(head, head)
    }

    pub fn grapheme_aligned(&self, _slice: RopeSlice) -> Self {
        // TODO: handle grapheme boundary
        Range {
            anchor: self.anchor,
            head: self.head,
            old_visual_position: self.old_visual_position,
        }
    }

    /// Compute a possibly new range from this range, attempting to ensure
    /// a minimum range width of 1 char by shifting the head in the forward
    /// direction as needed.
    ///
    /// This method will never shift the anchor, and will only shift the
    /// head in the forward direction.  Therefore, this method can fail
    /// at ensuring the minimum width if and only if the passed range is
    /// both zero-width and at the end of the `RopeSlice`.
    ///
    /// If the input range is grapheme-boundary aligned, the returned range
    /// will also be.  Specifically, if the head needs to shift to achieve
    /// the minimum width, it will shift to the next grapheme boundary.
    #[must_use]
    #[inline]
    pub fn min_width_1(&self, slice: RopeSlice) -> Self {
        if self.anchor == self.head {
            Range {
                anchor: self.anchor,
                head: next_grapheme_boundary(slice, self.head),
                old_visual_position: self.old_visual_position,
            }
        } else {
            *self
        }
    }

    /// Gets the left side position of the block cursor.
    pub fn cursor(self, text: RopeSlice) -> usize {
        if self.head > self.anchor {
            prev_grapheme_boundary(text, self.head)
        } else {
            self.head
        }
    }

    pub fn put_cursor(self, text: RopeSlice, char_idx: usize, extend: bool) -> Range {
        if extend {
            // TODO: handle other case;
            let anchor = self.anchor;
            Range::new(anchor, char_idx)
        } else {
            Range::point(char_idx)
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
    /// Ensures the selection adheres to the following invariants:
    /// 1. All ranges are grapheme aligned.
    /// 2. All ranges are at least 1charater wide, unless at the very end of the document.
    /// 3. Ranges are non-overlapping.
    /// 4. Ranges are sorrted by their position in the text.
    pub fn ensure_invariants(self, text: RopeSlice) -> Self {
        // TODO: call graphme_aligned
        self.transform(|r| r.min_width_1(text)).normalize()
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

    fn normalize(self) -> Self {
        // TODO: sort and handle overlap
        self
    }

    pub fn transform<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(Range) -> Range,
    {
        for range in self.ranges.iter_mut() {
            *range = f(*range)
        }
        self.normalize()
    }
}
