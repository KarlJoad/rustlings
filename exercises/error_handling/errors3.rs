// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

/* The ? operator will return erroneous conditions early.
 * This means that if total_cost returns an error, then main will return the error
 * immediately. However, if it is valid, then we only need to consider the correctly
 * functioning condition and how to handle it. */

use std::num::ParseIntError;

fn main() -> Result<String, ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        Ok(format!("You can't afford that many!"))
    } else {
        tokens -= cost;
        Ok(format!("You now have {} tokens.", tokens))
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
