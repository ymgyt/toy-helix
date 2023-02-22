use crate::view::ViewId;
use slotmap::HopSlotMap;

use super::{graphics::Rect, view::View};

#[derive(Debug)]
pub struct Tree {
    root: ViewId,

    pub focus: ViewId,
    area: Rect,

    nodes: HopSlotMap<ViewId, Node>,

    stack: Vec<(ViewId, Rect)>,
}

#[derive(Debug)]
pub struct Node {
    parent: ViewId,
    content: Content,
}

#[derive(Debug)]
pub enum Content {
    View(Box<View>),
    Container(Box<Container>),
}

impl Node {
    pub fn container(layout: Layout) -> Self {
        Self {
            parent: ViewId::default(),
            content: Content::Container(Box::new(Container::new(layout))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Container {
    layout: Layout,
    children: Vec<ViewId>,
    area: Rect,
}

impl Container {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            children: Vec::new(),
            area: Rect::default(),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new(Layout::Vertical)
    }
}

impl Tree {
    pub fn new(area: Rect) -> Self {
        let root = Node::container(Layout::Vertical);

        let mut nodes = HopSlotMap::with_key();
        let root = nodes.insert(root);

        // Root is it's own parent;
        nodes[root].parent = root;

        Self {
            root,
            focus: root,
            area,
            nodes,
            stack: Vec::new(),
        }
    }

    pub fn views(&self) -> impl Iterator<Item = (&View, bool)> {
        let focus = self.focus;
        self.nodes.iter().filter_map(move |(key, node)| match node {
            Node {
                content: Content::View(view),
                ..
            } => Some((view.as_ref(), focus == key)),
            _ => None,
        })
    }
}
