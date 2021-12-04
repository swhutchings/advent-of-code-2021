use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize)
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect_vec();

        let amount = parts[1].parse().unwrap();

        match parts[0] {
            "forward" => Ok(Command::Forward(amount)),
            "down" => Ok(Command::Down(amount)),
            "up" => Ok(Command::Up(amount)),
            &_ => Err(()),
        }
    }
}

fn main() {
    let instructions = include_str!("../input.txt")
        .lines()
        .map(|l| Command::from_str(l).unwrap())
        .collect_vec();

    println!("* Part 1");

    let mut horizontal  = 0;
    let mut depth = 0;

    for instruction in &instructions {
        match instruction {
            Command::Forward(x) => horizontal += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Product: {}", depth * horizontal);

    // Part 2

    println!("* Part 2");

    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for instruction in &instructions {
        match instruction {
            Command::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        }
    }

    println!("Aim: {}", aim);
    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Product: {}", depth * horizontal);
}

