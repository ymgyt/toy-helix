use crate::term::commands::MappableCommand;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyTrie {
    Leaf(MappableCommand),
}
