use std::any::Any;

use crate::view::editor::Editor;

use crate::tui::buffer::Buffer as Surface;
use crate::view::graphics::Rect;

pub struct Context<'a> {
    pub editor: &'a mut Editor,
}
pub trait Component: Any {
    /// Render the component onto the provided surface.
    fn render(&mut self, area: Rect, frame: &mut Surface, ctx: &mut Context);
}

pub struct Compositor {
    layers: Vec<Box<dyn Component>>,
    area: Rect,
}

impl Compositor {
    pub fn new(area: Rect) -> Self {
        Self {
            layers: Vec::new(),
            area,
        }
    }

    pub fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        for layer in &mut self.layers {
            layer.render(area, surface, cx);
        }
    }
}
