use algorithms_and_data_structures::algorithms::{
    bubble_sort::bubble_sort, linear_search::linear_search,
};

fn main() {
    println!("Hello, world!");
    let mut arr = [7, 5, 8, 1, 4, 6];
    let mut vec: Vec<u16> = vec![7, 5, 8, 1, 4, 6];
    let mut strs = ["hello", "world", "ante"];
    dbg!(linear_search(&arr, 5));
    bubble_sort(&mut arr);
    bubble_sort(&mut vec);
    bubble_sort(&mut strs);
    dbg!(arr);
    dbg!(vec);
    dbg!(strs);
}
