use std::error::Error;

pub mod errors;
mod one;


fn main() -> Result<(), Box<dyn Error>> {
    let answer = one::find_three_entries_that_sum_to_2020();
    println!("{:?}", answer?);
    Ok(())
}
