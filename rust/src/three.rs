use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAP_FILE: &'static str = "../inputs/3.1.txt";

pub(crate) fn num_trees_encountered(slope_right: usize, slope_down: usize) -> usize {
    let file = File::open(MAP_FILE).expect("Input file is missing!");
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .step_by(slope_down)
        .scan(0, |col, mut row| {
            let path_char = row.chars().cycle().nth(*col as usize);
            *col += slope_right;
            path_char
        })
        .filter(|&c| c == '#')
        .count()
}

pub(crate) fn multi_slope_tree_product() -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|slope| num_trees_encountered(slope.0, slope.1))
        .fold(1, |acc, n| n * acc)
}
