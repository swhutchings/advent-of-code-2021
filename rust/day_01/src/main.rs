use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;

fn main() {
    // Part 1
    let result = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("Part 1: {}", result);

    // Part 2
    let result = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("Part 2: {}", result);
}