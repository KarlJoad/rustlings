// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

/* The if-let and while-let leverage the fact that these blocks are considered
 * EXPRESSIONS. This does mean that if and while can be used on the RHS of an
 * assignment.
 * Here, the thing we are matching against and "deconstructing" is the parameter
 * to the if/while, and what we are comparing against is on the RHS, with the
 * alternatives inside blocks after that. */

fn main() {
    let optional_value = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // Remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
