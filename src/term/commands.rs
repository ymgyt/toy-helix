pub mod typed;

pub use typed::*;

use std::{fmt, num::NonZeroUsize};

use ropey::RopeSlice;

use crate::{
    core::{
        doc_formatter::TextFormat,
        movement::{move_horizontally, Direction, Movement},
        text_annotations::TextAnnotations,
        Range,
    },
    current,
    view::editor::Editor,
};

pub struct Context<'a> {
    // pub register: Option<char>,
    pub count: Option<NonZeroUsize>,
    pub editor: &'a mut Editor,
    // pub callback: Option<crate::compositor::Callback>,
    // pub on_next_key_callback: Option<Box<dyn FnOnce(&mut Context, KeyEvent)>>,
    // pub jobs: &'a mut Jobs,
}

impl<'a> Context<'a> {
    /// Returns 1 if no explicit count was provided
    pub fn count(&self) -> usize {
        self.count.map_or(1, |v| v.get())
    }
}

/// A MappbleCommand is either a static command like "jump_view_up" or a Typable command like
/// :format. It causes a side-effect on the state (usually by creating and applying a transaction).
/// Both of these types of commands can be mapped with keybindings in the config.toml.
#[derive(Clone)]
pub enum MappableCommand {
    Typable {
        name: String,
        args: Vec<String>,
        doc: String,
    },
    Static {
        name: &'static str,
        fun: fn(cx: &mut Context),
        doc: &'static str,
    },
}

macro_rules! static_commands {
    ( $($name:ident, $doc:literal,)* ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: Self = Self::Static {
                name: stringify!($name),
                fun: $name,
                doc: $doc,
            };
        )*

        pub const STATIC_COMMAND_LIST: &'static [Self] = &[
            $( Self::$name, )*
        ];
    }
}

impl MappableCommand {
    pub fn execute(&self, cx: &mut Context) {
        match &self {
            Self::Typable { .. } => todo!(),
            Self::Static { fun, .. } => (fun)(cx),
        }
    }
    pub fn name(&self) -> &str {
        match &self {
            Self::Typable { name, .. } => name,
            Self::Static { name, .. } => name,
        }
    }
    #[rustfmt::skip]
    static_commands!(
        no_op, "Do nothing",
        move_char_right, "Move right",
        move_char_left, "Move left",
        _quit, "Quit",
    );
}

impl fmt::Debug for MappableCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MappableCommand").field(&self.name()).finish()
    }
}

impl fmt::Display for MappableCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for MappableCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MappableCommand::Typable { name: first_name, .. }, MappableCommand::Typable { name: second_name, .. }) => {
                first_name == second_name
            }
            (MappableCommand::Static { name: first_name, .. }, MappableCommand::Static { name: second_name, .. }) => {
                first_name == second_name
            }
            _ => false,
        }
    }
}

fn no_op(_cx: &mut Context) {}

type MoveFn = fn(RopeSlice, Range, Direction, usize, Movement, &TextFormat, &mut TextAnnotations) -> Range;

fn move_impl(cx: &mut Context, move_fn: MoveFn, dir: Direction, behaviour: Movement) {
    // NOTE: count == 1 is very important. This value is used for neth_next_boundary, so if it is 0, the head will not move and will be buggy.
    let count = cx.count();
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);
    let text_fmt = doc.text_format(view.inner_area(doc).width, None);
    let mut annotations = view.text_annotations(doc, None);

    let selection = doc
        .selection(view.id)
        .clone()
        .transform(|range| move_fn(text, range, dir, count, behaviour, &text_fmt, &mut annotations));

    doc.set_selection(view.id, selection);
}

fn move_char_left(cx: &mut Context) {
    move_impl(cx, move_horizontally, Direction::Backward, Movement::Move)
}

fn move_char_right(cx: &mut Context) {
    move_impl(cx, move_horizontally, Direction::Forward, Movement::Move)
}

// for debug use.
fn _quit(cx: &mut Context) {
    panic!("Bye")
}
