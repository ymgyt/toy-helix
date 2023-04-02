use crate::{
    core::syntax::HighlightEvent,
    term::compositor::{Component, Context},
    view::{document::Document, editor::Editor, graphics::Rect, theme::Theme, view::View},
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

        // let mut highlights = Self::doc_syntax_highlights(doc, view.offset.anchor, inner.height, theme);

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
}

impl Component for EditorView {
    fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        tracing::info!("EditorView rendering...");
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
