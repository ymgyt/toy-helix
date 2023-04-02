use std::fmt;

use super::{document::Document, graphics::Rect, theme::Theme, DocumentId};
use crate::{core::text_annotations::TextAnnotations, view::ViewId};

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
}
