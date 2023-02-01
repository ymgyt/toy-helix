use std::fmt;

pub struct Context /*<'a>*/ {
    // pub register: Option<char>,
    // pub count: Option<NonZeroUsize>,
    // pub editor: &'a mut Editor,

    // pub callback: Option<crate::compositor::Callback>,
    // pub on_next_key_callback: Option<Box<dyn FnOnce(&mut Context, KeyEvent)>>,
    // pub jobs: &'a mut Jobs,
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
    pub fn name(&self) -> &str {
        match &self {
            Self::Typable { name, .. } => name,
            Self::Static { name, .. } => name,
        }
    }
    #[rustfmt::skip]
    static_commands!(
        no_op, "Do nothing",
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
