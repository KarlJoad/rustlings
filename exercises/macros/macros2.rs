// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

/* Macros in Rust behave like C functions, namely they must be declared (but not
 * necessarily defined) before they are used. */

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
