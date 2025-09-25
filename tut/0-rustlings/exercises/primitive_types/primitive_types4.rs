// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    /*  &a[1..4] -> produces a slice, which is a &[T]
        pointer to the first element and a length
        cannot bind a[1..4] since it would be a dynamically sized sequence
        of T elements. Compiler needs to have a compile-time known size
        for every type. &a[1..4] has a size of 2 * size_of(usize)
        since it stores a pointer and a length
    */
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
