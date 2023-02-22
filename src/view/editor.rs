use std::{collections::BTreeMap, path::Path, sync::Arc};

use arc_swap::access::DynAccess;

use crate::view::theme::{Theme, DEFAULT_THEME};

use super::{document::Document, graphics::Rect, tree::Tree, DocumentId};

pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub struct Editor {
    pub tree: Tree,
    pub documents: BTreeMap<DocumentId, Document>,
    pub config: Arc<dyn DynAccess<Config>>,
    pub exit_code: i32,
    pub theme: Theme,
}

impl Editor {
    pub fn new(mut area: Rect, config: Arc<dyn DynAccess<Config>>) -> Self {
        // TODO: load from loader;
        let theme = DEFAULT_THEME.clone();
        let tree = Tree::new(area);
        Self {
            tree,
            documents: BTreeMap::new(),
            config,
            exit_code: 0,
            theme,
        }
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
            doc.id
        };

        todo!()
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
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Load,
    Replace,
    HorizontalSplit,
    VerticalSplit,
}
