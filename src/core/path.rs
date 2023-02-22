use etcetera::home_dir;
use std::path::{Component, Path, PathBuf};

/// Expands tilde `~` into users home directory if available, otherwise returns the path
/// unchanged. The tilde will only be expanded when present as the first component of the path
/// and only slash follows it.
pub fn expand_tilde(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    if let Some(Component::Normal(c)) = components.peek() {
        if c == &"~" {
            if let Ok(home) = home_dir() {
                return home.join(path.strip_prefix("~").unwrap());
            }
        }
    }

    path.to_path_buf()
}

pub fn get_canonicalized_path(path: &Path) -> std::io::Result<PathBuf> {
    let path = expand_tilde(path);
    let path = if path.is_relative() {
        std::env::current_dir().map(|current_dir| current_dir.join(path))?
    } else {
        path
    };

    Ok(get_normalized_path(path.as_path()))
}

/// Normalize a path, removing things like `.` and `..`.
///
/// CAUTION: This does not resolve symlinks (unlike
/// [`std::fs::canonicalize`]). This may cause incorrect or surprising
/// behavior at times. This should be used carefully. Unfortunately,
/// [`std::fs::canonicalize`] can be hard to use correctly, since it can often
/// fail, or on Windows returns annoying device paths. This is a problem Cargo
/// needs to improve on.
/// Copied from cargo: <https://github.com/rust-lang/cargo/blob/070e459c2d8b79c5b2ac5218064e7603329c92ae/crates/cargo-util/src/paths.rs#L81>
pub fn get_normalized_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
