use aoc2022::{read_input, solution, Args, Solution};
use clap::Parser;

fn get_elves(data: String) -> Vec<i32> {
    let mut elves = vec![];
    let mut elf = 0;
    for line in data.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<i32>().unwrap();
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves
}

fn solve_part_a(elves: &[i32]) -> Option<i32> {
    Some(elves[0])
}

fn solve_part_b(elves: &[i32]) -> Option<i32> {
    Some(elves.iter().take(3).sum())
}

pub fn solve(data: String) -> Solution {
    let elves = get_elves(data);
    solution!(solve_part_a(&elves), solve_part_b(&elves), 1)
}

fn main() {
    let args = Args::parse();
    match read_input(1, args) {
        Ok(data) => println!("{}", solve(data)),
        Err(e) => eprintln!("Failed to read input file. {}", e),
    };
}
