#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));
}

#[macro_export]
macro_rules! keymap {
    (@trie $cmd:ident) => {
        $crate::term::keymap::KeyTrie::Leaf($crate::term::commands::MappableCommand::$cmd)
    };

    (@trie
        { $label:literal $(sticky=$sticky:literal)? $($($key:literal)|+ => $value:tt,)+ }
    ) => {
          keymap!({ $label $(sticky=$sticky)? $($($key)|+ => $value,)+ })
    };

    (
        { $label:literal $(sticky=$sticky:literal)? $($($key:literal)|+ => $value:tt,)+ }
    ) => {
        {
            let _cap = $crate::macros::hashmap!(@count $($($key),+),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            let mut _order = ::std::vec::Vec::with_capacity(_cap);
            $(
                $(
                    let _key = $key.parse::<$crate::view::input::KeyEvent>().unwrap();
                    let _duplicate = _map.insert(
                        _key,
                        keymap!(@trie $value)
                    );
                    assert!(_duplicate.is_none(), "Duplicate key found: {:?}", _duplicate.unwrap());
                    _order.push(_key);
                )+
            )*
            let mut _node = $crate::term::keymap::KeyTrieNode::new($label, _map, _order);
            $( _node.is_sticky = $sticky; )?
            $crate::term::keymap::KeyTrie::Node(_node)
        }
    };
}

pub use hashmap;
pub use keymap;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hashmap_count() {
        let n = hashmap!(@count "a","b","c");
        assert_eq!(n, 3);
    }

    #[test]
    fn keymap_1() {
        keymap!({ "Label" "a" => no_op, });
        keymap!({ "Label" "a" | "b" => no_op, });
    }

    #[test]
    fn keymap_2() {
        keymap!({ "Label"
            "a" => { "Nest"
                "b" => no_op,
                "c" => no_op,
            },
        });
    }
}
