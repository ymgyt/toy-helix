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
    (
        { $label:literal $($($key:literal)|+ => $value:tt,)+ }
    ) => {
        {
            let _cap = $crate::macros::hashmap!(@count $($($key),+),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            // let mut _order = ::std::vec::Vec::with_capacity(_cap);
            $(
                $(
                    let _key = $key.parse::<$crate::view::input::KeyEvent>().unwrap();
                    let _duplicate = _map.insert(
                        _key,
                        keymap!(@trie $value)
                    );
                )+
            )*
        }
    }
}

pub use {hashmap, keymap};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hashmap_count() {
        let n = hashmap!(@count "a","b","c");
        assert_eq!(n, 3);
    }

    #[test]
    fn hashmap_1() {
        keymap!({ "Label" "a" => no_op, });
    }
}
