// #![feature(trace_macros)]
use toy_helix::macros::{hashmap, keymap};
fn main() {
    // trace_macros!(true);
    keymap!({"Normal mode"
        "i" => no_op,
        "a" | "b" => no_op,
    });
}
