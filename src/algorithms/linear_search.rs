pub fn linear_search<T: PartialEq>(slice: &[T], x: T) -> Option<usize> {
    for (index, item) in slice.iter().enumerate() {
        if *item == x {
            return Some(index);
        }
    }
    None
}
