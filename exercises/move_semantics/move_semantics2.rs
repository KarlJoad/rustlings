// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

/* Here, fill_vec borrows vec0, and creates a new vector that will be owned by main.
 * This way, both vec0 and vec1 exist at the same time, but vec1 is somewhat independent
 * of vec0. */

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut newvec = Vec::new();

    newvec.push(22);
    newvec.push(44);
    newvec.push(66);

    newvec
}
