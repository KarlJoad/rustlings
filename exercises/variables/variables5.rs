// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

/* A let binding can shadow outer/earlier bindings. Thus, the second number, the
 * one that has the value of the integer number 3 is the only one available after
 * that line, and the number with the value of a string 3 is no longer available. */
fn main() {
    let number = "3"; // don't change this line
    println!("Number {}", number);
    let number = 3;
    println!("Number {}", number);
}
