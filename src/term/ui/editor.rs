use crate::{
    core::syntax::{self, HighlightEvent},
    term::compositor::{Component, Context},
    view::{
        document::{Document, Mode},
        editor::{CursorShapeConfig, Editor},
        graphics::Rect,
        theme::Theme,
        view::View,
    },
};

use crate::tui::buffer::Buffer as Surface;

use super::document::render_document;

pub struct EditorView {}

impl EditorView {
    pub fn new() -> Self {
        Self {}
    }
}

impl EditorView {
    pub fn render_view(
        &self,
        editor: &Editor,
        doc: &Document,
        view: &View,
        viewport: Rect,
        surface: &mut Surface,
        is_focused: bool,
    ) {
        let inner = view.inner_area(doc);
        let area = view.area;
        let theme = &editor.theme;
        let config = editor.config();

        let text_annotations = view.text_annotations(doc, Some(theme));

        let mut highlights = Self::doc_syntax_highlights(doc, view.offset.anchor, inner.height, theme);

        let highlights: Box<dyn Iterator<Item = HighlightEvent>> = if is_focused {
            let highlights = syntax::merge(
                highlights,
                Self::doc_selection_highlights(editor.mode(), doc, view, theme, &config.cursor_shape),
            );
            // TODO highlight focused view element
            Box::new(highlights)
        } else {
            Box::new(highlights)
        };

        render_document(
            surface,
            inner,
            doc,
            view.offset,
            &text_annotations,
            // highlights,
            theme,
            // &mut line_decorations,
            // &mut translated_positions,
        )
    }

    /// Get syntax highlights for a document in a view represented by the first line
    /// and column (`offset`) and the last line. This is done instead of using a view
    /// directly to enable rendering syntax highlighted docs anywhere (eg. picker preview)
    pub fn doc_syntax_highlights<'doc>(
        doc: &'doc Document,
        anchor: usize,
        height: u16,
        _theme: &Theme,
    ) -> Box<dyn Iterator<Item = HighlightEvent> + 'doc> {
        let text = doc.text().slice(..);
        let row = text.char_to_line(anchor.min(text.len_chars()));

        let range = {
            // Calculate viewport byte ranges:
            // Saturaging subs to make it inclusive zero indexing.
            let last_line = text.len_lines().saturating_sub(1);
            let last_visible_line = (row + height as usize).saturating_sub(1).min(last_line);
            let start = text.line_to_byte(row.min(last_line));
            let end = text.line_to_byte(last_visible_line + 1);

            start..end
        };
        // TODO: handle syntax
        // doc.syntax()
        Box::new(
            [HighlightEvent::Source {
                start: text.byte_to_char(range.start),
                end: text.byte_to_char(range.end),
            }]
            .into_iter(),
        )
    }

    pub fn doc_selection_highlights(
        mode: Mode,
        doc: &Document,
        view: &View,
        theme: &Theme,
        cursor_shape_config: &CursorShapeConfig,
    ) -> Vec<(usize, std::ops::Range<usize>)> {
        todo!()
    }
}

impl Component for EditorView {
    fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        surface.set_style(area, cx.editor.theme.get("ui.background"));

        // TODO: buffer line
        // -1 for command line
        let mut editor_area = area.clip_bottom(1);

        // TODO: editor.resize

        for (view, is_focused) in cx.editor.tree.views() {
            let doc = cx.editor.document(view.doc).unwrap();
            self.render_view(cx.editor, doc, view, area, surface, is_focused);
        }

        // TODO
    }
}
