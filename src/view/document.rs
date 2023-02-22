use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::access::DynAccess;

use crate::core::{encoding, Rope};

use super::{editor::Config, DocumentId};

pub struct Document {
    pub id: DocumentId,
    text: Rope,

    path: Option<PathBuf>,
}

impl Document {
    pub fn open(
        path: &Path,
        encoding: Option<&'static encoding::Encoding>,
        // config_laoder: Option<Arc<syntax::Loader>>,
        config: Arc<dyn DynAccess<Config>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
