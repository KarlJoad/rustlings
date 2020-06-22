// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

/* By default, all variables behave like in functional programming languages,
 * where the value of the variable is immutable. We must explicitly state that
 * the variable is mutable to change it after its initial binding. */

fn main() {
    let mut x: u32 = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
