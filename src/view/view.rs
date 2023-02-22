use std::fmt;

use super::{graphics::Rect, DocumentId};
use crate::view::ViewId;

#[derive(Clone)]
pub struct View {
    pub id: ViewId,
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
