use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &'static str = "../inputs/5.1.txt";

pub fn seat_ids() -> Result<(i32, i32), Box<dyn Error>> {
    let passes = BufReader::new(File::open(INPUT)?).lines();
    let mut seat_ids = vec![];
    for pass in passes {
        // Find the row
        let (mut row_lower, mut row_upper) = (0, 127);
        let line = pass.unwrap();
        for char in line.chars().take(7) {
            let mid = row_lower + (row_upper - row_lower) / 2;
            match char {
                'F' => row_upper = mid,
                'B' => row_lower = mid + 1,
                _ => panic!("Unexpected character {} in boarding pass", char),
            }
        }

        // Find the column
        let (mut col_lower, mut col_upper) = (0, 7);
        for char in line.chars().skip(7).take(3) {
            let mid = col_lower + (col_upper - col_lower) / 2;
            match char {
                'L' => col_upper = mid,
                'R' => col_lower = mid + 1,
                _ => panic!("Unexpected character {} in boarding pass", char),
            }
        }

        seat_ids.push(8 * row_lower + col_lower);
    }

    let copy_seats: HashSet<&i32> = seat_ids.iter().collect();
    let seat_range = *seat_ids.iter().min().unwrap()..*seat_ids.iter().max().unwrap();
    let mut my_seat = 0;
    for seat in seat_range {
        if !copy_seats.contains(&seat) {
            my_seat = seat;
        }
    }
    Ok((*seat_ids.iter().max().unwrap(), my_seat))
}
