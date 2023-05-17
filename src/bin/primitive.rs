use std::iter;
use arrow::array::Int16Array;

fn main() {

    let nums_1:Vec<i16> = (0..100).into_iter().collect();
    let int_array_1 = Int16Array::from_iter_values(nums_1.into_iter());
    println!("{int_array_1:?}");

    let nums_2:Vec<i16> = (0..100).into_iter().collect();
    let int_array_2=Int16Array::from(nums_2);
    println!("{int_array_2:?}");
}