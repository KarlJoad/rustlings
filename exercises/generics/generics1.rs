// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

/* I feel like this may be the solution, but the _ gives little constraint on
 * what may be present in the list. If there were additional items, we could
 * constrain the possible types available in the list more.
 * However, with the current usage, a Vec<&str> is appropriate.*/

fn main() {
    let mut shopping_list: Vec<_> = Vec::new();
    shopping_list.push("milk");
}
