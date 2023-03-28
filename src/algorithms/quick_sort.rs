#![allow(dead_code)]

fn quick_sort<T: PartialOrd + Clone>(slice: &mut [T]) {
    qs(slice, 0, slice.len() - 1);
}

fn qs<T: PartialOrd + Clone>(slice: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(slice, lo, hi);

    qs(slice, lo, pivot_idx - 1);
    qs(slice, pivot_idx + 1, hi)
}

fn partition<T: PartialOrd + Clone>(slice: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot_pos = hi;
    let pivot = slice[pivot_pos].clone();

    let lo_i32: i32 = lo.try_into().unwrap();
    let mut idx = lo_i32 - 1;
    for i in lo..hi {
        if slice[i] <= pivot {
            idx += 1;
            slice.swap(i, idx.try_into().unwrap());
        }
    }
    idx += 1;
    slice.swap(pivot_pos, idx.try_into().unwrap());

    idx.try_into().unwrap()
}

#[test]
fn test_quick_sort() {
    let mut vec: Vec<u16> = vec![1, 69, 4, 5, 420, 6, 7, 8];
    quick_sort(&mut vec);
    assert_eq!(vec, vec![1, 4, 5, 6, 7, 8, 69, 420]);
}
