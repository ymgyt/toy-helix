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

    /// May be used by the parent component to compute the child area.
    /// viewport is the maximum allowd area, and the child should stay widthin those bounds.
    ///
    /// The returned size might be larger than the vieport if the child is too big to fit.
    /// Int this case the parent can use the values to calculate scroll.
    fn required_size(&mut self, _viewport: (u16, u16)) -> Option<(u16, u16)> {
        None
    }
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

    pub fn size(&self) -> Rect {
        self.area
    }

    /// Add a layer to be rendered in fromt of all existing layers.
    pub fn push(&mut self, mut layer: Box<dyn Component>) {
        let size = self.size();
        layer.required_size((size.width, size.height));
        self.layers.push(layer);
    }
}
