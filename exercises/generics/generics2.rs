// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

/* We need a type parameter, or a Generic type specified for Wrapper, for the
 * single field the struct has.
 * Then, we must implement a generic version of Wrapper. It must take in something
 * of type T, and will return a Wrapper around that type T. */

struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value,  42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
