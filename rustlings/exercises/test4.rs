// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

#[macro_use]
mod my_mod {
    macro_rules! my_macro {
        () => {
            println!("no arguments");
        };
        ($expression: expr) => {
            concat!("Hello ", stringify!($expression))
        };
    }
}
fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
