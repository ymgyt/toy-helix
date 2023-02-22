use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::core::position::Position;

#[derive(Default)]
pub struct Args {
    pub files: Vec<(PathBuf, Position)>,
}

impl Args {
    pub fn parse_args() -> Result<Args> {
        let mut args = Args::default();
        let mut argv = std::env::args().peekable();

        argv.next(); // skip the program, we don't care about that

        while let Some(arg) = argv.next() {
            match arg.as_str() {
                arg if arg.starts_with("--") => {
                    anyhow::bail!("unexpected double dash argment: {}", arg)
                }
                arg => args.files.push(parse_file(arg)),
            }
        }

        // MEMO: currently do not drain argv. should we impl ?

        Ok(args)
    }
}

fn parse_file(s: &str) -> (PathBuf, Position) {
    let def = || (PathBuf::from(s), Position::default());
    if Path::new(s).exists() {
        return def();
    }
    // TODO: handle row only case, file.rs:10
    split_path_row_col(s).unwrap_or_else(def)
}

/// Split file.rs:10:2 into PathBuf , row and col.
fn split_path_row_col(s: &str) -> Option<(PathBuf, Position)> {
    let mut s = s.rsplitn(3, ':');
    let col: usize = s.next()?.parse().ok()?;
    let row: usize = s.next()?.parse().ok()?;
    let path = s.next()?.into();
    let pos = Position::new(row.saturating_sub(1), col.saturating_sub(1));
    Some((path, pos))
}
