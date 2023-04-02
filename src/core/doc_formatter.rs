use std::{borrow::Cow, mem::replace};

use crate::core::syntax::Highlight;
use ropey::RopeSlice;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

use super::{
    graphemes::{Grapheme, GraphemeStr, RopeGraphemes},
    position::Position,
    text_annotations::TextAnnotations,
};

#[derive(Debug, Clone, Copy)]
pub enum GraphemeSource {
    Document {
        codepoints: u32,
    },
    /// Inline virtual text can not be highlighted with a `Highlight` iterator
    /// because it's not part of the document. Instead the `Highlight`
    /// is emitted right by the document formatter
    VirtualText {
        highlight: Option<Highlight>,
    },
}

#[derive(Debug, Clone)]
pub struct FormattedGrapheme<'a> {
    pub grapheme: Grapheme<'a>,
    pub source: GraphemeSource,
}

impl<'a> FormattedGrapheme<'a> {
    pub fn new(g: GraphemeStr<'a>, visual_x: usize, tab_width: u16, source: GraphemeSource) -> FormattedGrapheme<'a> {
        FormattedGrapheme {
            grapheme: Grapheme::new(g, visual_x, tab_width),
            source,
        }
    }
    pub fn doc_chars(&self) -> usize {
        match self.source {
            GraphemeSource::Document { codepoints } => codepoints as usize,
            GraphemeSource::VirtualText { .. } => 0,
        }
    }

    pub fn width(&self) -> usize {
        self.grapheme.width()
    }
}

#[derive(Debug, Clone)]
pub struct TextFormat {
    pub soft_wrap: bool,
    pub tab_width: u16,
    pub max_wrap: u16,
    pub max_indent_retain: u16,
    pub wrap_indicator: Box<str>,
    pub wrap_indicator_highlight: Option<Highlight>,
    pub viewport_width: u16,
}

// test implementation is basically only used for testing or when softwrap is always disabled
impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            soft_wrap: false,
            tab_width: 4,
            max_wrap: 3,
            max_indent_retain: 4,
            wrap_indicator: Box::from(" "),
            viewport_width: 17,
            wrap_indicator_highlight: None,
        }
    }
}

#[derive(Debug)]
pub struct DocumentFormatter<'t> {
    text_fmt: &'t TextFormat,
    annotations: &'t TextAnnotations,

    /// The visual position at the end of the last yielded word boundary
    visual_pos: Position,
    graphemes: RopeGraphemes<'t>,
    /// The character pos of the `graphemes` iter used for inserting annotations
    char_pos: usize,
    /// The line pos of the `graphemes` iter used for inserting annotations
    line_pos: usize,
    exhausted: bool,

    /// Line breaks to be reserved for virtual text
    /// at the next line break
    virtual_lines: usize,
    inline_anntoation_graphemes: Option<(Graphemes<'t>, Option<Highlight>)>,

    // softwrap specific
    /// The indentation of the current line
    /// Is set to `None` if the indentation level is not yet known
    /// because no non-whitespace graphemes have been encountered yet
    indent_level: Option<usize>,
    /// In case a long word needs to be split a single grapheme might need to be wrapped
    /// while the rest of the word stays on the same line
    peeked_grapheme: Option<(FormattedGrapheme<'t>, usize)>,
    /// A first-in first-out (fifo) buffer for the Graphemes of any given word
    word_buf: Vec<FormattedGrapheme<'t>>,
    /// The index of the next grapheme that will be yielded from the `word_buf`
    word_i: usize,
}

impl<'t> DocumentFormatter<'t> {
    /// Creates a new formatter at the last block before `char_idx`.
    /// A block is a chunk which always ends with a linebreak.
    /// This is usually just a normal line break.
    /// However very long lines are always wrapped at constant intervals that can be cheaply calculated
    /// to avoid pathological behaviour.
    pub fn new_at_prev_checkpoint(
        text: RopeSlice<'t>,
        text_fmt: &'t TextFormat,
        annotations: &'t TextAnnotations,
        char_idx: usize,
    ) -> (Self, usize) {
        // TODO divide long lines into blocks to avoid bad performance for long lines
        let block_line_idx = text.char_to_line(char_idx.min(text.len_chars()));
        let block_char_idx = text.line_to_char(block_line_idx);
        annotations.reset_pos(block_char_idx);
        (
            DocumentFormatter {
                text_fmt,
                annotations,
                visual_pos: Position { row: 0, col: 0 },
                graphemes: RopeGraphemes::new(text.slice(block_char_idx..)),
                char_pos: block_char_idx,
                exhausted: false,
                virtual_lines: 0,
                indent_level: None,
                peeked_grapheme: None,
                word_buf: Vec::with_capacity(64),
                word_i: 0,
                line_pos: block_line_idx,
                inline_anntoation_graphemes: None,
            },
            block_char_idx,
        )
    }

    fn next_inline_annotation_grapheme(&mut self) -> Option<(&'t str, Option<Highlight>)> {
        return None;
        // TODO
        /*
        loop {
            if let Some(&mut (ref mut annotation, highlight)) = self.inline_anntoation_graphemes.as_mut() {
                if let Some(grapheme) = annotation.next() {
                    return Some((grapheme, highlight));
                }
            }

            // if let Some((annotation, highlight)) = self.annotations.next_inline_annotation_at(self.char_pos) {
            //     self.inline_anntoation_graphemes = Some((UnicodeSegmentation::graphemes(&*annotation.text, true), highlight))
            // } else {
            //     return None;
            // }
        }
        */
    }

    fn advance_grapheme(&mut self, col: usize) -> Option<FormattedGrapheme<'t>> {
        let (grapheme, source) = if let Some((grapheme, highlight)) = self.next_inline_annotation_grapheme() {
            (grapheme.into(), GraphemeSource::VirtualText { highlight })
        } else if let Some(grapheme) = self.graphemes.next() {
            // self.virtual_lines += self.annotations.annotation_lines_at(self.char_pos);
            let codepoints = grapheme.len_chars() as u32;

            // let overlay = self.annotations.overlay_at(self.char_pos);
            // let grapheme = match overlay {
            //     Some((overlay, _)) => overlay.grapheme.as_str().into(),
            //     None => Cow::from(grapheme).into(),
            // };
            let grapheme: GraphemeStr<'_> = Cow::from(grapheme).into();

            self.char_pos += codepoints as usize;
            (grapheme, GraphemeSource::Document { codepoints })
        } else {
            if self.exhausted {
                return None;
            }
            self.exhausted = true;
            // EOF grapheme is required for rendering
            // and correct position computations
            return Some(FormattedGrapheme {
                grapheme: Grapheme::Other { g: " ".into() },
                source: GraphemeSource::Document { codepoints: 0 },
            });
        };

        let grapheme = FormattedGrapheme::new(grapheme, col, self.text_fmt.tab_width, source);

        Some(grapheme)
    }

    /// returns the document line pos of the **next** grapheme that will be yielded
    pub fn line_pos(&self) -> usize {
        self.line_pos
    }
}

impl<'t> Iterator for DocumentFormatter<'t> {
    type Item = (FormattedGrapheme<'t>, Position);

    fn next(&mut self) -> Option<Self::Item> {
        let grapheme = if self.text_fmt.soft_wrap {
            todo!()
            // if self.word_i >= self.word_buf.len() {
            //     self.advance_to_next_word();
            //     self.word_i = 0;
            // }
            // let grapheme = replace(self.word_buf.get_mut(self.word_i)?, FormattedGrapheme::placeholder());
            // self.word_i += 1;
            // grapheme
        } else {
            self.advance_grapheme(self.visual_pos.col)?
        };

        let pos = self.visual_pos;
        if grapheme.grapheme == Grapheme::Newline {
            self.visual_pos.row += 1;
            self.visual_pos.row += std::mem::take(&mut self.virtual_lines);
            self.visual_pos.col = 0;
            self.line_pos += 1;
        } else {
            self.visual_pos.col += grapheme.width();
        }
        Some((grapheme, pos))
    }
}
