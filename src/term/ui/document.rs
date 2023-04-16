use crate::{
    core::{
        doc_formatter::{DocumentFormatter, TextFormat},
        graphemes::Grapheme,
        position::{visual_offset_from_block, Position},
        syntax::{Highlight, HighlightEvent},
        text_annotations::TextAnnotations,
        RopeSlice,
    },
    tui::buffer::Buffer as Surface,
    view::{
        document::Document,
        editor::{WhitespaceConfig, WhitespaceRenderValue},
        graphics::{Rect, Style},
        theme::Theme,
        view::ViewPosition,
    },
};

/// A wrapper around a HighlightIterator
/// that merges the layered highlights to create the final text style
/// and yields the active text style and the char_idx where the active
/// style will have to be recomputed.
struct StyleIter<'a, H: Iterator<Item = HighlightEvent>> {
    text_style: Style,
    active_highlights: Vec<Highlight>,
    highlight_iter: H,
    theme: &'a Theme,
}

impl<H: Iterator<Item = HighlightEvent>> Iterator for StyleIter<'_, H> {
    type Item = (Style, usize);
    fn next(&mut self) -> Option<(Style, usize)> {
        while let Some(event) = self.highlight_iter.next() {
            tracing::info!("{event:?}");
            match event {
                HighlightEvent::HighlightStart(highlights) => self.active_highlights.push(highlights),
                HighlightEvent::HighlightEnd => {
                    self.active_highlights.pop();
                }
                HighlightEvent::Source { start, end } => {
                    if start == end {
                        continue;
                    }
                    let style = self
                        .active_highlights
                        .iter()
                        .fold(self.text_style, |acc, span| acc.patch(self.theme.highlight(span.0)));
                    return Some((style, end));
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LinePos {
    /// Indicates whether the given visual line
    /// is the first visual line of the given document line
    pub first_visual_line: bool,
    /// The line index of the document line that contains the given visual line
    pub doc_line: usize,
    /// Vertical offset from the top of the inner view area
    pub visual_line: u16,
    /// The first char index of this visual line.
    /// Note that if the visual line is entirely filled by
    /// a very long inline virtual text then this index will point
    /// at the next (non-virtual) char after this visual line
    pub start_char_idx: usize,
}

#[allow(clippy::too_many_arguments)]
pub fn render_document(
    surface: &mut Surface,
    viewport: Rect,
    doc: &Document,
    offset: ViewPosition,
    doc_annotations: &TextAnnotations,
    highlight_iter: impl Iterator<Item = HighlightEvent>,
    theme: &Theme,
    // line_decorations: &mut [Box<dyn LineDecoration +'_>],
    // translated_positions: &mut [TranslatedPosition],
) {
    tracing::info!("===========Render_document=============");
    tracing::info!("viewport={viewport:?} offse={offset:?}");
    let mut renderer = TextRenderer::new(surface, doc, theme, offset.horizontal_offset, viewport);
    render_text(
        &mut renderer,
        doc.text().slice(..),
        offset,
        &doc.text_format(viewport.width, Some(theme)),
        doc_annotations,
        highlight_iter,
        theme,
        // line_decorations,
        // translated_positions,
    )
}

#[derive(Debug)]
pub struct TextRenderer<'a> {
    pub surface: &'a mut Surface,
    pub text_style: Style,
    pub whitespace_style: Style,
    pub indent_guide_char: String,
    pub indent_guide_style: Style,
    pub newline: String,
    pub nbsp: String,
    pub space: String,
    pub tab: String,
    pub tab_width: u16,
    pub starting_indent: usize,
    pub draw_indent_guides: bool,
    pub col_offset: usize,
    pub viewport: Rect,
}

impl<'a> TextRenderer<'a> {
    pub fn new(surface: &'a mut Surface, doc: &Document, theme: &Theme, col_offset: usize, viewport: Rect) -> TextRenderer<'a> {
        let editor_config = doc.config.load();
        let WhitespaceConfig {
            render: ws_render,
            characters: ws_chars,
        } = &editor_config.whitespace;

        let tab_width = doc.tab_width();
        // TODO: handle render tab
        let tab = " ".repeat(tab_width);
        let newline = " ".to_owned();
        let space = " ".to_owned();
        let nbsp = " ".to_owned();

        let text_style = theme.get("ui.text");

        TextRenderer {
            surface,
            indent_guide_char: '|'.into(),
            newline,
            nbsp,
            space,
            tab_width: tab_width as u16,
            tab,
            whitespace_style: theme.get("ui.virtual.whitespace"),
            starting_indent: 2,
            indent_guide_style: theme.get("ui.virtual.whitespace"),
            text_style,
            draw_indent_guides: false,
            viewport,
            col_offset,
        }
    }

    /// Draws a single `grapheme` at the current render position with a specified `style`.
    pub fn draw_grapheme(
        &mut self,
        grapheme: Grapheme,
        mut style: Style,
        last_indent_level: &mut usize,
        is_in_indent_area: &mut bool,
        position: Position,
    ) {
        let cut_off_start = self.col_offset.saturating_sub(position.col);
        let is_whitespace = grapheme.is_whitespace();

        if is_whitespace {
            style = style.patch(self.whitespace_style);
        }

        let width = grapheme.width();
        let space = &self.space;
        let nbsp = &self.nbsp;
        let tab = &self.tab;
        let grapheme = match grapheme {
            Grapheme::Tab { width } => {
                // let grapheme_tab_width = char_to_byte_idx(tab, width);
                // &tab[..grapheme_tab_width]
                &tab[..]
            }
            // TODO special rendering for other whitespaces?
            Grapheme::Other { ref g } if g == " " => space,
            Grapheme::Other { ref g } if g == "\u{00A0}" => nbsp,
            Grapheme::Other { ref g } => g,
            Grapheme::Newline => &self.newline,
        };

        let in_bounds = self.col_offset <= position.col && position.col < self.viewport.width as usize + self.col_offset;

        if in_bounds {
            self.surface.set_string(
                self.viewport.x + (position.col - self.col_offset) as u16,
                self.viewport.y + position.row as u16,
                grapheme,
                style,
            );
        } else if cut_off_start != 0 && cut_off_start < width {
            // TODO:
            todo!()
        }

        // TODO: handle indent_level
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_text<'t>(
    renderer: &mut TextRenderer,
    text: RopeSlice<'t>,
    offset: ViewPosition,
    text_fmt: &TextFormat,
    text_annotations: &TextAnnotations,
    highlight_iter: impl Iterator<Item = HighlightEvent>,
    theme: &Theme,
    // line_decorations: &mut [Box<dyn LineDecoration + '_>],
    // translated_position: &mut [TranslatedPosition],
) {
    let (Position { row: mut row_off, .. }, mut char_pos) =
        visual_offset_from_block(text, offset.anchor, offset.anchor, text_fmt, text_annotations);
    row_off += offset.vertical_offset;
    assert_eq!(0, offset.vertical_offset);

    let (mut formatter, mut first_visible_char_idx) =
        DocumentFormatter::new_at_prev_checkpoint(text, text_fmt, text_annotations, offset.anchor);
    let mut styles = StyleIter {
        text_style: renderer.text_style,
        active_highlights: Vec::with_capacity(64),
        highlight_iter,
        theme,
    };

    let mut last_line_pos = LinePos {
        first_visual_line: false,
        doc_line: usize::MAX,
        visual_line: u16::MAX,
        start_char_idx: usize::MAX,
    };
    let mut is_in_indent_area = true;
    let mut last_line_indent_level = 0;
    let mut style_span = styles.next().unwrap_or_else(|| (Style::default(), usize::MAX));

    loop {
        let doc_line = formatter.line_pos();
        let Some((grapheme, mut pos)) = formatter.next() else {
            // TODO handle this case
            break;
        };

        // TODO: check pos.row < row_off

        // if the end of the viewport is reached stop rendering
        if pos.row as u16 >= renderer.viewport.height {
            break;
        }

        // acquire the correct grapheme style
        if char_pos >= style_span.1 {
            style_span = styles.next().unwrap_or((Style::default(), usize::MAX));
        }
        char_pos += grapheme.doc_chars();

        let grapheme_style = style_span.0;

        renderer.draw_grapheme(
            grapheme.grapheme,
            grapheme_style,
            &mut last_line_indent_level,
            &mut is_in_indent_area,
            pos,
        );
    }
}
