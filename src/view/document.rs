use anyhow::{bail, Context, Error, Result};
use arc_swap::access::DynAccess;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use crate::core::{
    doc_formatter::TextFormat, encoding, syntax::LanguageConfiguration, text_annotations::TextAnnotations, Range, Rope,
    RopeBuilder, Selection,
};

use super::{editor::Config, theme::Theme, DocumentId, ViewId};

/// 8kB of buffer space for encoding and decoding Repos.
const BUF_SIZE: usize = 8192;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal => f.write_str("normal"),
            Mode::Select => f.write_str("select"),
            Mode::Insert => f.write_str("insert"),
        }
    }
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Mode::Normal),
            "select" => Ok(Mode::Select),
            "insert" => Ok(Mode::Insert),
            _ => bail!("Invalid mode '{}'", s),
        }
    }
}

pub struct Document {
    pub id: DocumentId,
    text: Rope,
    selections: HashMap<ViewId, Selection>,

    path: Option<PathBuf>,
    encoding: &'static encoding::Encoding,

    language: Option<Arc<LanguageConfiguration>>,

    pub config: Arc<dyn DynAccess<Config>>,
}

pub fn from_reader<R: std::io::Read + ?Sized>(
    reader: &mut R,
    encoding: Option<&'static encoding::Encoding>,
) -> Result<(Rope, &'static encoding::Encoding)> {
    let mut buf = [0u8; BUF_SIZE];
    let mut buf_out = [0u8; BUF_SIZE];
    let mut builder = RopeBuilder::new();

    let (encoding, mut decoder, mut slice, mut is_empty) = {
        let read = reader.read(&mut buf)?;
        let is_empty = read == 0;
        let encoding = encoding.unwrap_or_else(|| {
            let mut encoding_detector = chardetng::EncodingDetector::new();
            encoding_detector.feed(&buf, is_empty);
            encoding_detector.guess(None, true)
        });
        let decoder = encoding.new_decoder();

        let slice = &buf[..read];
        (encoding, decoder, slice, is_empty)
    };

    let buf_str = unsafe { std::str::from_utf8_unchecked_mut(&mut buf_out[..]) };
    let mut total_written = 0usize;
    loop {
        let mut total_read = 0usize;

        loop {
            let (result, read, written, ..) =
                decoder.decode_to_str(&slice[total_read..], &mut buf_str[total_written..], is_empty);

            total_read += read;
            total_written += written;
            match result {
                encoding::CoderResult::InputEmpty => {
                    debug_assert_eq!(slice.len(), total_read);
                    break;
                }
                encoding::CoderResult::OutputFull => {
                    debug_assert!(slice.len() > total_read);
                    builder.append(&buf_str[..total_written]);
                    total_written = 0;
                }
            }
        }

        if is_empty {
            debug_assert_eq!(reader.read(&mut buf)?, 0);
            builder.append(&buf_str[..total_written]);
            break;
        }

        let read = reader.read(&mut buf)?;
        slice = &buf[..read];
        is_empty = read == 0;
    }
    let rope = builder.finish();
    Ok((rope, encoding))
}

impl Document {
    pub fn from(text: Rope, encoding: Option<&'static encoding::Encoding>, config: Arc<dyn DynAccess<Config>>) -> Self {
        let encoding = encoding.unwrap_or(encoding::UTF_8);

        Self {
            id: DocumentId::default(),
            text,
            selections: HashMap::default(),
            path: None,
            encoding,
            language: None,
            config,
        }
    }
    /// Create a new document from path. ENcoding is auto-detected, but it can be manually
    /// overwritten with the encoding parameter.
    pub fn open(
        path: &Path,
        encoding: Option<&'static encoding::Encoding>,
        // config_laoder: Option<Arc<syntax::Loader>>,
        config: Arc<dyn DynAccess<Config>>,
    ) -> anyhow::Result<Self> {
        let (rope, encoding) = if path.exists() {
            let mut file = std::fs::File::open(path).context(format!("unable to open {path:?}"))?;
            from_reader(&mut file, encoding)?
        } else {
            bail!("open file which does not exists not implemented")
        };

        let mut doc = Self::from(rope, Some(encoding), config);

        doc.set_path(Some(path))?;
        // TODO: detect language
        // TODO: detect indent and line ending

        Ok(doc)
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn set_path(&mut self, path: Option<&Path>) -> std::result::Result<(), std::io::Error> {
        let path = path.map(crate::core::path::get_canonicalized_path).transpose()?;

        self.path = path;

        Ok(())
    }

    pub fn language_config(&self) -> Option<&LanguageConfiguration> {
        self.language.as_deref()
    }

    pub fn text(&self) -> &Rope {
        &self.text
    }

    pub fn tab_width(&self) -> usize {
        // TODO: get from language config
        4
    }

    pub fn set_selection(&mut self, view_id: ViewId, selection: Selection) {
        self.selections
            .insert(view_id, selection.ensure_invariants(self.text().slice(..)));
    }

    /// Find the origin selection of the text in a document, i.e. where
    /// a single cursor sould go if it were on the first grapheme. If
    /// the text is empty, returns (0, 0).
    pub fn origin(&self) -> Range {
        if self.text().len_chars() == 0 {
            return Range::new(0, 0);
        }

        Range::new(0, 1).grapheme_aligned(self.text().slice(..))
    }

    pub fn reset_selection(&mut self, view_id: ViewId) {
        let origin = self.origin();
        self.set_selection(view_id, Selection::single(origin.anchor, origin.head));
    }

    /// Initializes a new selection for the given view if it does not already have one.
    pub fn ensure_view_init(&mut self, view_id: ViewId) {
        if self.selections.get(&view_id).is_none() {
            self.reset_selection(view_id);
        }
    }

    pub fn text_format(&self, mut viewport_width: u16, theme: Option<&Theme>) -> TextFormat {
        // TODO: handle language config

        let config = self.config.load();
        // let soft_wrap = &config.soft_wrap;
        let tab_width = self.tab_width() as u16;

        TextFormat {
            soft_wrap: false,
            tab_width,
            max_wrap: viewport_width / 4,
            max_indent_retain: viewport_width * 2 / 5,
            wrap_indicator: "".to_owned().into_boxed_str(),
            wrap_indicator_highlight: None,
            viewport_width,
        }
    }

    pub fn text_annotations(&self, _theme: Option<&Theme>) -> TextAnnotations {
        TextAnnotations::default()
    }
}
