use crate::{
    term::compositor::{Component, Context},
    view::graphics::Rect,
};

use crate::tui::buffer::Buffer as Surface;

pub struct EditorView {}

impl EditorView {
    pub fn new() -> Self {
        Self {}
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
            // self.render_view(cx.editor, doc, view, area, surface, is_focused);
        }

        todo!()
    }
}
