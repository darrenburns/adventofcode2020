use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

const INPUT_FILE: &'static str = "../inputs/6.1.txt";

// Part 1
pub(crate) fn sum_of_yes_counts_by_anyone() -> Result<usize, Box<dyn Error>> {
    Ok(fs::read_to_string(INPUT_FILE)?
        .split("\n\n")
        .map(|group| group.replace("\n", ""))
        .map(|line| line.chars().collect::<HashSet<char>>())
        .map(|set| set.len())
        .sum())
}

// Part 2
pub(crate) fn sum_of_yes_counts_by_everyone() -> Result<usize, Box<dyn Error>> {
    let groups = fs::read_to_string(INPUT_FILE)?;
    let mut yes_count_sum = 0;
    for group in groups.split("\n\n") {
        let group_lines: Vec<&str> = group.split("\n").collect();
        let mut person_answer_sets = vec![];
        // Convert each person's answers to a set
        for line in group_lines {
            let answer_set = line.chars().collect::<HashSet<char>>();
            if answer_set.len() > 0 {
                person_answer_sets.push(answer_set);
            }
        }

        // Add the length of the intersection of all these answer sets to the overall sum
        let all_chars = "abcdefghijklmnopqrstuvwxyz"
            .to_string()
            .chars()
            .collect::<HashSet<char>>();

        yes_count_sum += person_answer_sets
            .iter()
            .fold(all_chars, |mut acc, ans_set| {
                acc = acc.intersection(ans_set).copied().collect();
                acc
            })
            .len();
    }

    Ok(yes_count_sum)
}
