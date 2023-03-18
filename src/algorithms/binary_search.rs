pub fn binary_search<T: PartialOrd>(sorted_slice: &[T], x: T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high = sorted_slice.len();
    while low < high {
        let mid = (low + high) / 2;

        if sorted_slice[mid] == x {
            return Some(mid);
        } else if sorted_slice[mid] > x {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    None
}

#[test]
fn test_binary_search() {
    let vec: Vec<u16> = vec![1, 4, 5, 6, 7, 8];
    assert_eq!(binary_search(&vec, 1), Some(0));
    assert_eq!(binary_search(&vec, 8), Some(5));
    assert_eq!(binary_search(&vec, 6), Some(3));
}
