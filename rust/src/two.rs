use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn get_valid_password_count_part_1() -> usize {
    get_valid_password_count(&has_valid_letter_counts)
}

pub(crate) fn get_valid_password_count_part_2() -> usize {
    get_valid_password_count(&has_valid_letter_positions)
}


fn get_valid_password_count(valid_pass_fn: &dyn Fn(&String) -> Option<bool>) -> usize {
    let file = File::open("../inputs/2.1.txt").unwrap();
    BufReader::new(file)
        .lines()
        .filter(|l| valid_pass_fn(l.as_ref().unwrap()).unwrap())
        .count()
}

fn has_valid_letter_counts(line: &String) -> Option<bool> {
    if let Some((letter, password, min, max)) = parse_line(line) {
        let num_chars = password.chars()
            .filter(|l| l == &letter)
            .count();
        return Some(num_chars >= min && num_chars <= max);
    }
    Some(false)
}

fn has_valid_letter_positions(line: &String) -> Option<bool> {
    if let Some((letter, password, min, max)) = parse_line(line) {
        let min_char = password.chars().nth(min - 1)?;
        let max_char = password.chars().nth(max - 1)?;
        return Some((min_char == letter && max_char != letter) ||
            (min_char != letter && max_char == letter));
    }
    Some(false)
}

fn parse_line(line: &String) -> Option<(char, &str, usize, usize)> {
    let mut split_line = line.split(" ");

    let mut first_part = split_line.next()?.split("-");
    let letter = split_line.next()?.chars().next().unwrap();
    let password = split_line.next()?;

    let min = first_part.next()?.parse::<usize>().unwrap();
    let max = first_part.next()?.parse::<usize>().unwrap();
    Some((letter, password, min, max))
}
