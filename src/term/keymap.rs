pub mod default;
pub mod macros;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use arc_swap::access::DynAccess;

use crate::{
    term::commands::MappableCommand,
    view::{document::Mode, input::KeyEvent},
};

pub use default::default;

#[derive(Debug, Clone)]
pub struct KeyTrieNode {
    name: String,
    map: HashMap<KeyEvent, KeyTrie>,
    order: Vec<KeyEvent>,
    pub is_sticky: bool,
}

impl KeyTrieNode {
    pub fn new(name: &str, map: HashMap<KeyEvent, KeyTrie>, order: Vec<KeyEvent>) -> Self {
        Self {
            name: name.to_string(),
            map,
            order,
            is_sticky: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for KeyTrieNode {
    fn default() -> Self {
        Self::new("", HashMap::new(), Vec::new())
    }
}

impl PartialEq for KeyTrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl Deref for KeyTrieNode {
    type Target = HashMap<KeyEvent, KeyTrie>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for KeyTrieNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyTrie {
    Leaf(MappableCommand),
    Node(KeyTrieNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeymapResult {
    /// Needs more keys to execute a command. Contains valid keys for next keystroke.
    Pending(KeyTrieNode),
    Matched(MappableCommand),
    /// Matched a sequence of commands to execute.
    MatchedSequence(Vec<MappableCommand>),
    /// Key was not found in the root keymap
    NotFound,
    /// Key is invalid in combination with previous keys. Contains keys leading upto
    /// and including current (invalid) key.
    Cancelled(Vec<KeyEvent>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keymap {
    /// Always a Node
    root: KeyTrie,
}

impl Keymap {
    pub fn new(root: KeyTrie) -> Self {
        Keymap { root }
    }
}

pub struct Keymaps {
    pub map: Box<dyn DynAccess<HashMap<Mode, Keymap>>>,
    state: Vec<KeyEvent>,
    pub sticky: Option<KeyTrieNode>,
}

impl Keymaps {
    pub fn new(map: Box<dyn DynAccess<HashMap<Mode, Keymap>>>) -> Self {
        Self {
            map,
            state: Vec::new(),
            sticky: None,
        }
    }
}
