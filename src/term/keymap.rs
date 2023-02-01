use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{term::commands::MappableCommand, view::input::KeyEvent};

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
