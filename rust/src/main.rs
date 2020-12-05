use std::error::Error;

pub mod errors;
mod one;
mod two;
mod three;
mod four;
mod five;


fn main() -> Result<(), Box<dyn Error>> {
    let d1p1_answer = one::find_two_entries_that_sum_to_2020();
    println!("Day 1 Part 1: {:?}", d1p1_answer?);
    let d1p2_answer = one::find_three_entries_that_sum_to_2020();
    println!("Day 1 Part 2: {:?}", d1p2_answer?);

    let valid_password_count1 = two::get_valid_password_count_part_1();
    println!("Day 2 Part 1: {:?}", valid_password_count1);
    let valid_password_count2 = two::get_valid_password_count_part_2();
    println!("Day 2 Part 2: {:?}", valid_password_count2);
    let num_trees = three::num_trees_encountered(3, 1);
    println!("Day 3 Part 1: {}", num_trees);

    let part_two = three::multi_slope_tree_product();
    println!("Day 3 Part 2: {}", part_two);

    let num_valid_passports = four::get_num_valid_passports();
    println!("Day 4, Part 1: {:?}", num_valid_passports?);

    let (highest_seat, my_seat) = five::seat_ids()?;
    println!("Day 5, Part 1: {:?}", highest_seat);
    println!("Day 5, Part 2: {:?}", my_seat);

    Ok(())
}
