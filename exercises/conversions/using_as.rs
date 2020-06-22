// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.

/* Here, there is no definition of how to divide a f64 by a usize value.
 * So, because WE, as programmers, know that x/2 is nearly the same as
 * x/2.0, the division should work the same. However, this only holds
 * true because the divisor is the float and dividend is the integer.
 * This statement cannot be held true in the other case, because of floating-point
 * accuracy problems. */

// The goal is to make sure that the division does not fail to compile
fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()
        .fold(0.0, |a, b| a + b);
    total / (values.len() as f64)
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}
