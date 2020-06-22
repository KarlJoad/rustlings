// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

/* If you create an overloaded macro (one that can take different types of arguments
 * which all share the same name), then you must have a separator between the
 * alternative macro bodies. */

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
