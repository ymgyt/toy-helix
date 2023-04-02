use crate::view::ViewId;
use slotmap::HopSlotMap;

use super::{graphics::Rect, view::View};

#[derive(Debug)]
pub struct Tree {
    root: ViewId,

    pub focus: ViewId,
    area: Rect,

    pub nodes: HopSlotMap<ViewId, Node>,

    // used for traversals
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

    pub fn view(view: View) -> Self {
        Self {
            parent: ViewId::default(),
            content: Content::View(Box::new(view)),
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

    /// Get reference to a View by index.
    /// # Panics
    ///
    /// Panics if index is not in self.nodes, or if the node's content is not Content::View.
    pub fn get(&self, index: ViewId) -> &View {
        self.try_get(index).unwrap()
    }

    pub fn try_get(&self, index: ViewId) -> Option<&View> {
        match &self.nodes[index] {
            Node {
                content: Content::View(view),
                ..
            } => Some(view),
            _ => None,
        }
    }

    /// Get a mutable reference to a View by index.
    /// # Panics
    ///
    /// Panics if `index` is not in self.nodes, or it the node's content is not Content::View.
    /// This can be checked with Self::contains.
    pub fn get_mut(&mut self, index: ViewId) -> &mut View {
        match &mut self.nodes[index] {
            Node {
                content: Content::View(view),
                ..
            } => view,
            _ => unreachable!(),
        }
    }

    pub fn split(&mut self, view: View, layout: Layout) -> ViewId {
        let focus = self.focus;
        let parent = self.nodes[focus].parent;

        let node = Node::view(view);
        let node = self.nodes.insert(node);
        self.get_mut(node).id = node;

        let container = match &mut self.nodes[parent] {
            Node {
                content: Content::Container(container),
                ..
            } => container,
            _ => unreachable!(),
        };
        if container.layout == layout {
            let pos = if container.children.is_empty() {
                0
            } else {
                let pos = container.children.iter().position(|&child| child == focus).unwrap();
                pos + 1
            };
            container.children.insert(pos, node);
            self.nodes[node].parent = parent;
        } else {
            panic!("layout that does not match container is not implemented yet");
        };

        // focus the new node
        self.focus = node;

        self.recalculate();

        node
    }

    pub fn is_empty(&self) -> bool {
        match &self.nodes[self.root] {
            Node {
                content: Content::Container(container),
                ..
            } => container.children.is_empty(),
            _ => unreachable!(),
        }
    }

    pub fn recalculate(&mut self) {
        if self.is_empty() {
            // There are no more views, so the tree should focus iteself again.
            self.focus = self.root;

            return;
        }

        self.stack.push((self.root, self.area));

        // take the area
        // fetch the node
        // a) node is view, give it whole area
        // b) node is container, calculate reas for each child and push them on the stack

        while let Some((key, area)) = self.stack.pop() {
            let node = &mut self.nodes[key];

            match &mut node.content {
                Content::View(view) => {
                    view.area = area;
                }
                Content::Container(container) => {
                    container.area = area;

                    match container.layout {
                        Layout::Horizontal => {
                            let len = container.children.len();
                            let height = area.height / len as u16;
                            let mut child_y = area.y;
                            for (i, child) in container.children.iter().enumerate() {
                                let mut area = Rect::new(container.area.x, child_y, container.area.width, height);
                                child_y += height;

                                if i == len - 1 {
                                    area.height = container.area.y + container.area.height - area.y;
                                }

                                self.stack.push((*child, area));
                            }
                        }
                        Layout::Vertical => {
                            let len = container.children.len();

                            let width = area.width / len as u16;

                            let inner_gap = 1u16;

                            let mut child_x = area.x;

                            for (i, child) in container.children.iter().enumerate() {
                                let mut area = Rect::new(child_x, container.area.y, width, container.area.height);
                                child_x += width + inner_gap;

                                if i == len - 1 {
                                    area.width = container.area.x + container.area.width - area.x;
                                }

                                self.stack.push((*child, area));
                            }
                        }
                    }
                }
            }
        }
    }
}
