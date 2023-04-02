#[macro_export]
macro_rules! doc_mut {
    ($editor:expr, $id:expr) => {{
        $editor.documents.get_mut($id).unwrap()
    }};
    ($editor:expr) => {{
        notimplemented!()
    }};
}

/// Get the current view immutably
/// Returns &View
#[macro_export]
macro_rules! view {
    ($editor:expr, $id:expr) => {{
        $editor.tree.get($id)
    }};
    ($editor:expr) => {{
        $editor.tree.get($editor.tree.focus)
    }};
}
