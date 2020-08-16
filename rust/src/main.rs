use std::rc::Rc;
use std::cell::RefCell;

mod problems;


fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    println!("Running sum problem: {:?}", problems::running_sum_of_array::running_sum(nums));

    let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
    let extra_candies: i32 = 3;
    println!("Extra candies problem: {:?}", problems::greatest_number_of_candies::kids_with_candies(candies, extra_candies));

    let input_str: String = "codeleet".to_string();
    let str_indices: Vec<i32> = vec![4, 5, 6, 7, 0, 2, 1, 3];
    println!("Shuffle string problem: {}", problems::shuffle_string::restore_string(input_str, str_indices));

    let num: i32 = 234;
    println!("Product minus sum problem: {}", problems::product_minus_sum::subtract_product_and_sum(num));

    let tree = Some(Rc::new(RefCell::new(problems::bst_range_sum::TreeNode {
        val: 0,
        left: None,
        right: None,
    })));
    println!("BST range sum problem: {}", problems::bst_range_sum::range_sum_bst(tree, 1, 10));
}
