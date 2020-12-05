use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::errors::InvalidInput;

fn get_expenses() -> Vec<i64> {
    let file = File::open("../inputs/1.1.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap_or_default())
        .collect::<Vec<i64>>()
}

// Part 1
pub(crate) fn find_two_entries_that_sum_to_2020() -> Result<i64, Box<dyn Error>> {
    let expenses = get_expenses();
    let num_entries = expenses.len();

    for first_entry_index in 0..num_entries {
        for second_entry_idx in 0..num_entries {
            if first_entry_index != second_entry_idx {
                let first_num = expenses.get(first_entry_index).unwrap();
                let second_num = expenses.get(second_entry_idx).unwrap();
                if first_num + second_num == 2020 {
                    return Ok(first_num * second_num);
                }
            }
        }
    }
    Err(Box::new(InvalidInput))
}


// Part 2
pub(crate) fn find_three_entries_that_sum_to_2020() -> Result<i64, Box<dyn Error>> {
    let expenses = get_expenses();
    let num_entries = expenses.len();

    for first_entry_index in 0..num_entries {
        for second_entry_idx in 0..num_entries {
            for third_entry_idx in 0..num_entries {
                if first_entry_index != second_entry_idx &&
                    first_entry_index != third_entry_idx &&
                    second_entry_idx != third_entry_idx {
                    let first_num = expenses.get(first_entry_index).unwrap();
                    let second_num = expenses.get(second_entry_idx).unwrap();
                    let third_num = expenses.get(third_entry_idx).unwrap();
                    if first_num + second_num + third_num == 2020 {
                        return Ok(first_num * second_num * third_num);
                    }
                }
            }
        }
    }
    Err(Box::new(InvalidInput))
}
