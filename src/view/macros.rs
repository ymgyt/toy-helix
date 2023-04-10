#[macro_export]
macro_rules! current {
    ($editor:expr) => {{
        let view = $crate::view_mut!($editor);
        let id = view.doc;
        let doc = $crate::doc_mut!($editor, &id);
        (view, doc)
    }};
}

#[macro_export]
macro_rules! doc_mut {
    ($editor:expr, $id:expr) => {{
        $editor.documents.get_mut($id).unwrap()
    }};
    ($editor:expr) => {{
        notimplemented!()
    }};
}

#[macro_export]
macro_rules! view_mut {
    ($editor:expr, $id:expr) => {{
        $editor.tree.get_mut($id)
    }};
    ($editor:expr) => {{
        $editor.tree.get_mut($editor.tree.focus)
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
