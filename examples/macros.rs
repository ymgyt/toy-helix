// #![feature(trace_macros)]
use toy_helix::macros::{hashmap, keymap};
fn main() {
    // trace_macros!(true);
    let m = keymap!({"Normal mode"
        "i" => no_op,
        "a" | "b" => no_op,
        "g" => { "Goto"
            "g" => no_op,
        },
    });

    println!("{m:#?}");
}
