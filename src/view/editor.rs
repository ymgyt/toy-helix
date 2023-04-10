use arc_swap::access::{DynAccess, DynGuard};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, num::NonZeroUsize, path::Path, pin::Pin, sync::Arc};
use tokio::time::{sleep, Duration, Instant, Sleep};

use crate::view::{
    document::{Document, Mode},
    graphics::{CursorKind, Rect},
    theme::{Theme, DEFAULT_THEME},
    tree::{Layout, Tree},
    view::View,
    DocumentId,
};

// Cursor shape is read and used on every rendered frame and so needs
// to be fast. Therefore we avoid a hashmap and use an enum indexed array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorShapeConfig([CursorKind; 3]);

impl CursorShapeConfig {
    pub fn from_mode(&self, mode: Mode) -> CursorKind {
        self.get(mode as usize).copied().unwrap_or_default()
    }
}

impl std::ops::Deref for CursorShapeConfig {
    type Target = [CursorKind; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for CursorShapeConfig {
    fn default() -> Self {
        Self([CursorKind::Block; 3])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// #[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct Config {
    // #[serde(default)]
    pub whitespace: WhitespaceConfig,
    /// Shape for cursor in each mode
    pub cursor_shape: CursorShapeConfig,
    /// Time in milliseconds since last keypress before idle timers trigger.
    /// used for autocompletion, set to 0 for instant.
    pub idle_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            whitespace: WhitespaceConfig::default(),
            cursor_shape: CursorShapeConfig::default(),
            idle_timeout: Duration::from_millis(400),
        }
    }
}

pub struct Editor {
    pub mode: Mode,
    pub tree: Tree,
    pub next_document_id: DocumentId,
    pub documents: BTreeMap<DocumentId, Document>,

    pub count: Option<std::num::NonZeroUsize>,

    pub config: Arc<dyn DynAccess<Config>>,
    pub exit_code: i32,
    pub theme: Theme,

    pub idle_timer: Pin<Box<Sleep>>,
}

impl Editor {
    pub fn new(mut area: Rect, config: Arc<dyn DynAccess<Config>>) -> Self {
        let conf = config.load();

        // TODO: load from loader;
        let theme = DEFAULT_THEME.clone();
        let tree = Tree::new(area);
        Self {
            mode: Mode::Normal,
            tree,
            next_document_id: DocumentId::default(),
            documents: BTreeMap::new(),
            count: None,
            config,
            exit_code: 0,
            theme,
            idle_timer: Box::pin(sleep(conf.idle_timeout)),
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn reset_idle_timer(&mut self) {
        let config = self.config();
        self.idle_timer.as_mut().reset(Instant::now() + config.idle_timeout);
    }

    pub fn open(&mut self, path: &Path, action: Action) -> anyhow::Result<DocumentId> {
        let path = crate::core::path::get_canonicalized_path(path)?;
        let id = self.document_by_path(&path).map(|doc| doc.id);

        let id = if let Some(id) = id {
            id
        } else {
            let mut doc = Document::open(
                &path,
                None,
                // Some(self.syn_loader.clone())
                self.config.clone(),
            )?;

            // TODO: handle diff

            let id = self.new_document(doc);

            // TODO: launch_language_server
            // let _ = self.launch_language_server(id);

            id
        };

        self.switch(id, action);
        Ok(id)
    }

    #[inline]
    pub fn document(&self, id: DocumentId) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn documents(&self) -> impl Iterator<Item = &Document> {
        self.documents.values()
    }

    pub fn document_by_path<P: AsRef<Path>>(&self, path: P) -> Option<&Document> {
        self.documents()
            .find(|doc| doc.path().map(|p| p == path.as_ref()).unwrap_or(false))
    }

    /// Generate an id for a new document and register it.
    fn new_document(&mut self, mut doc: Document) -> DocumentId {
        let id = self.next_document_id;
        self.next_document_id = DocumentId(unsafe { NonZeroUsize::new_unchecked(self.next_document_id.0.get() + 1) });
        doc.id = id;
        self.documents.insert(id, doc);

        // TODO: handle save queues

        id
    }

    pub fn config(&self) -> DynGuard<Config> {
        self.config.load()
    }

    pub fn switch(&mut self, id: DocumentId, action: Action) {
        // use crate::view::tree::Layout;

        if !self.documents.contains_key(&id) {
            tracing::error!("cannot switch to document that does not exist (anymore)");
            return;
        }

        // self.enter_normal_mode();

        match action {
            Action::HorizontalSplit | Action::VerticalSplit => {
                let view = self
                    .tree
                    .try_get(self.tree.focus)
                    .filter(|v| id == v.doc)
                    .cloned()
                    .unwrap_or_else(|| View::new(id));
                let view_id = self.tree.split(
                    view,
                    match action {
                        Action::HorizontalSplit => Layout::Horizontal,
                        Action::VerticalSplit => Layout::Vertical,
                        _ => unreachable!(),
                    },
                );
                let doc = doc_mut!(self, &id);
                doc.ensure_view_init(view_id);
            }
            Action::Load => {
                let view_id = view!(self).id;
                let doc = doc_mut!(self, &id);
                doc.ensure_view_init(view_id);
                return;
            }
            action => tracing::error!("action: {action:?} not implemented yet"),
        }

        // self_refresh();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Load,
    Replace,
    HorizontalSplit,
    VerticalSplit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhitespaceConfig {
    pub render: WhitespaceRender,
    pub characters: WhitespaceCharacters,
}

impl Default for WhitespaceConfig {
    fn default() -> Self {
        Self {
            render: WhitespaceRender::Basic(WhitespaceRenderValue::None),
            characters: WhitespaceCharacters::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhitespaceRender {
    Basic(WhitespaceRenderValue),
    Specific {
        default: Option<WhitespaceRenderValue>,
        space: Option<WhitespaceRenderValue>,
        nbsp: Option<WhitespaceRenderValue>,
        tab: Option<WhitespaceRenderValue>,
        newline: Option<WhitespaceRenderValue>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WhitespaceRenderValue {
    None,
    // TODO
    // Selection,
    All,
}

impl WhitespaceRender {
    pub fn space(&self) -> WhitespaceRenderValue {
        match *self {
            Self::Basic(val) => val,
            Self::Specific { default, space, .. } => space.or(default).unwrap_or(WhitespaceRenderValue::None),
        }
    }
    pub fn nbsp(&self) -> WhitespaceRenderValue {
        match *self {
            Self::Basic(val) => val,
            Self::Specific { default, nbsp, .. } => nbsp.or(default).unwrap_or(WhitespaceRenderValue::None),
        }
    }
    pub fn tab(&self) -> WhitespaceRenderValue {
        match *self {
            Self::Basic(val) => val,
            Self::Specific { default, tab, .. } => tab.or(default).unwrap_or(WhitespaceRenderValue::None),
        }
    }
    pub fn newline(&self) -> WhitespaceRenderValue {
        match *self {
            Self::Basic(val) => val,
            Self::Specific { default, newline, .. } => newline.or(default).unwrap_or(WhitespaceRenderValue::None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WhitespaceCharacters {
    pub space: char,
    pub nbsp: char,
    pub tab: char,
    pub tabpad: char,
    pub newline: char,
}

impl Default for WhitespaceCharacters {
    fn default() -> Self {
        Self {
            space: '·',    // U+00B7
            nbsp: '⍽',    // U+237D
            tab: '→',     // U+2192
            newline: '⏎', // U+23CE
            tabpad: ' ',
        }
    }
}
