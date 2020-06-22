// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

/* They mean that the string "Bar" is appended to the vector. NOT that "Bar" is
 * appended to each string in the vector. */

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    // fn append_bar(self) -> Self {
    // 	self.iter().map(|s| s.to_owned() + "Bar").collect()
    // }
    fn append_bar(mut self) -> Self {
	self.push(String::from("Bar"));
	return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

}
