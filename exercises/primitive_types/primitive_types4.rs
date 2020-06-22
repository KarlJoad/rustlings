// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // Uses similar syntax as Haskell
    // 1..4 => [1, 4), non-inclusive end. Arrays start index at 0.

    assert_eq!([2, 3, 4], nice_slice)
}
