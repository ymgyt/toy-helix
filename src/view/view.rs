use std::fmt;

use super::{document::Document, graphics::Rect, theme::Theme, DocumentId};
use crate::{
    core::{
        position::{char_idx_at_visual_offset, visual_offset_from_anchor, visual_offset_from_block},
        text_annotations::TextAnnotations,
        VisualOffsetError::{PosAfterMaxRow, PosBeforeAnchorRow},
    },
    view::ViewId,
};

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct ViewPosition {
    pub anchor: usize,
    pub horizontal_offset: usize,
    pub vertical_offset: usize,
}

#[derive(Clone)]
pub struct View {
    pub id: ViewId,
    pub offset: ViewPosition,
    pub area: Rect,
    pub doc: DocumentId,
}

impl fmt::Debug for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("View")
            .field("id", &self.id)
            .field("area", &self.area)
            .field("doc", &self.doc)
            .finish()
    }
}

impl View {
    pub fn new(doc: DocumentId) -> Self {
        Self {
            id: ViewId::default(),
            doc,
            offset: ViewPosition {
                anchor: 0,
                horizontal_offset: 0,
                vertical_offset: 0,
            },
            area: Rect::default(), // will get calculated upon inserting into tree
        }
    }

    pub fn inner_area(&self, doc: &Document) -> Rect {
        self.area.clip_left(self.gutter_offset(doc)).clip_bottom(1) // -1 for statusline
    }

    pub fn gutter_offset(&self, _doc: &Document) -> u16 {
        // TODO: impl
        2
    }

    pub fn text_annotations(&self, doc: &Document, theme: Option<&Theme>) -> TextAnnotations {
        doc.text_annotations(theme)
        // TODO
    }

    pub fn offset_coords_to_in_view_center<const CENTERING: bool>(
        &self,
        doc: &Document,
        scrolloff: usize,
    ) -> Option<ViewPosition> {
        let doc_text = doc.text().slice(..);
        let viewport = self.inner_area(doc);
        let vertical_viewport_end = self.offset.vertical_offset + viewport.height as usize;
        let text_fmt = doc.text_format(viewport.width, None);
        let annotations = self.text_annotations(doc, None);

        // - 1 so we have at least one gap in the middle.
        // a height of 6 with padding of 3 on each side will keep shifting the view back and forth
        // as we type
        let scrolloff = if CENTERING {
            0
        } else {
            scrolloff.min(viewport.height.saturating_sub(1) as usize / 2)
        };

        let cursor = doc.selection(self.id).primary().cursor(doc_text);
        let mut offset = self.offset;
        let off = visual_offset_from_anchor(
            doc_text,
            offset.anchor,
            cursor,
            &text_fmt,
            &annotations,
            vertical_viewport_end,
        );

        let (new_anchor, at_top) = match off {
            Ok((visual_pos, _)) if visual_pos.row < scrolloff + offset.vertical_offset => {
                if CENTERING {
                    // cursor out of view
                    return None;
                }
                (true, true)
            }
            Ok((visual_pos, _)) if visual_pos.row + scrolloff >= vertical_viewport_end => (true, false),
            Ok((_, _)) => (false, false),
            Err(_) if CENTERING => return None,
            Err(PosBeforeAnchorRow) => (true, true),
            Err(PosAfterMaxRow) => (true, false),
        };

        if new_anchor {
            let v_off = if at_top {
                scrolloff as isize
            } else {
                viewport.height as isize - scrolloff as isize - 1
            };
            (offset.anchor, offset.vertical_offset) =
                char_idx_at_visual_offset(doc_text, cursor, -v_off, 0, &text_fmt, &annotations);
        }

        if text_fmt.soft_wrap {
            offset.horizontal_offset = 0;
        } else {
            // determine the current visual column of the text
            let col = off
                .unwrap_or_else(|_| visual_offset_from_block(doc_text, offset.anchor, cursor, &text_fmt, &annotations))
                .0
                .col;

            let last_col = offset.horizontal_offset + viewport.width.saturating_sub(1) as usize;
            if col > last_col.saturating_sub(scrolloff) {
                // scroll right
                offset.horizontal_offset += col - (last_col.saturating_sub(scrolloff))
            } else if col < offset.horizontal_offset + scrolloff {
                // scroll left
                offset.horizontal_offset = col.saturating_sub(scrolloff)
            };
        }

        // if we are not centering return None if view position is unchanged
        if !CENTERING && offset == self.offset {
            return None;
        }

        Some(offset)
    }

    pub fn ensure_cursor_in_view(&mut self, doc: &Document, scrolloff: usize) {
        if let Some(offset) = self.offset_coords_to_in_view_center::<false>(doc, scrolloff) {
            self.offset = offset;
        }
    }
}
