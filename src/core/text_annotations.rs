use std::{cell::Cell, convert::identity, rc::Rc};

use super::{syntax::Highlight, Tendril};
/// An inline annotation is continuous text shown
/// on the screen before the grapheme that starts at
/// `char_idx`
#[derive(Debug, Clone)]
pub struct InlineAnnotation {
    pub text: Tendril,
    pub char_idx: usize,
}

/// Represents a **single Grapheme** that is part of the document
/// that start at `char_idx` that will be replaced with
/// a different `grapheme`.
/// If `grapheme` contains multiple graphemes the text
/// will render incorrectly.
/// If you want to overlay multiple graphemes simply
/// use multiple `Overlays`.
///
/// # Examples
///
/// The following examples are valid overlays for the following text:
///
/// `aX͎̊͢͜͝͡bc`
///
/// ```
/// use helix_core::text_annotations::Overlay;
///
/// // replaces a
/// Overlay {
///   char_idx: 0,
///   grapheme: "X".into(),
/// };
///
/// // replaces X͎̊͢͜͝͡
/// Overlay{
///   char_idx: 1,
///   grapheme: "\t".into(),
/// };
///
/// // replaces b
/// Overlay{
///   char_idx: 6,
///   grapheme: "X̢̢̟͖̲͌̋̇͑͝".into(),
/// };
/// ```
///
/// The following examples are invalid uses
///
/// ```
/// use helix_core::text_annotations::Overlay;
///
/// // overlay is not aligned at grapheme boundary
/// Overlay{
///   char_idx: 3,
///   grapheme: "x".into(),
/// };
///
/// // overlay contains multiple graphemes
/// Overlay{
///   char_idx: 0,
///   grapheme: "xy".into(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Overlay {
    pub char_idx: usize,
    pub grapheme: Tendril,
}

/// Line annotations allow for virtual text between normal
/// text lines. They cause `height` empty lines to be inserted
/// below the document line that contains `anchor_char_idx`.
///
/// These lines can be filled with text in the rendering code
/// as their contents have no effect beyond visual appearance.
///
/// To insert a line after a document line simply set
/// `anchor_char_idx` to `doc.line_to_char(line_idx)`
#[derive(Debug, Clone)]
pub struct LineAnnotation {
    pub anchor_char_idx: usize,
    pub height: usize,
}

#[derive(Debug)]
struct Layer<A, M> {
    annotations: Rc<[A]>,
    current_index: Cell<usize>,
    metadata: M,
}

impl<A, M: Clone> Clone for Layer<A, M> {
    fn clone(&self) -> Self {
        Layer {
            annotations: self.annotations.clone(),
            current_index: self.current_index.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<A, M> Layer<A, M> {
    pub fn reset_pos(&self, char_idx: usize, get_char_idx: impl Fn(&A) -> usize) {
        let new_index = self
            .annotations
            .binary_search_by_key(&char_idx, get_char_idx)
            .unwrap_or_else(identity);

        self.current_index.set(new_index);
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextAnnotations {
    inline_annotations: Vec<Layer<InlineAnnotation, Option<Highlight>>>,
    overlays: Vec<Layer<Overlay, Option<Highlight>>>,
    line_annotations: Vec<Layer<LineAnnotation, ()>>,
}

impl TextAnnotations {
    /// Prepare the TextAnnotations for iteration starting at char_idx
    pub fn reset_pos(&self, char_idx: usize) {
        reset_pos(&self.inline_annotations, char_idx, |annot| annot.char_idx);
        reset_pos(&self.overlays, char_idx, |annot| annot.char_idx);
        reset_pos(&self.line_annotations, char_idx, |annot| annot.anchor_char_idx);
    }
}

fn reset_pos<A, M>(layers: &[Layer<A, M>], pos: usize, get_pos: impl Fn(&A) -> usize) {
    for layer in layers {
        layer.reset_pos(pos, &get_pos)
    }
}
