#![feature(iter_array_chunks)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use aoc2022::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let solutions = vec![
        day01::make_solution(),
        day02::make_solution(),
        day03::make_solution(),
        day04::make_solution(),
        day05::make_solution(),
        day06::make_solution(),
    ];

    match args.day {
        Some(day) => match solutions.iter().find(|s| s.day == day) {
            Some(sol) => sol.solve(args.test_data),
            None => eprintln!("No solution for day {day}"),
        },
        None => {
            for sol in solutions {
                sol.solve(args.test_data);
            }
        }
    };
}
