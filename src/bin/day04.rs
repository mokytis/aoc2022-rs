use aoc2022::{read_input, solution, Args, Solution};
use clap::Parser;

fn parse_group(group: &str) -> (u32, u32) {
    group
        .split_once("-")
        .map(|v| (v.0.parse().unwrap(), v.1.parse().unwrap()))
        .unwrap()
}

//struct Line((u32, u32), (u32, u32));

struct Line {
    part_a: Range,
    part_b: Range,
}

struct Range {
    start: u32,
    end: u32,
}

impl Line {
    fn from_string(input: &str) -> Line {
        input
            .split_once(",")
            .map(|(left, right)| (parse_group(left), parse_group(right)))
            .unwrap()
            .into()
    }
    fn parts_are_subset(&self) -> bool {
        (self.part_a.start <= self.part_b.start && self.part_a.end >= self.part_b.end)
            || (self.part_a.start >= self.part_b.start && self.part_a.end <= self.part_b.end)
    }

    fn parts_overlap(&self) -> bool {
        !((self.part_a.start < self.part_b.start && self.part_a.end < self.part_b.start)
            || (self.part_a.start > self.part_b.end && self.part_a.end > self.part_b.end))
    }
}

impl From<(u32, u32)> for Range {
    fn from(data: (u32, u32)) -> Range {
        let (start, end) = data;
        Range {
            start: start,
            end: end,
        }
    }
}
impl From<((u32, u32), (u32, u32))> for Line {
    fn from(data: ((u32, u32), (u32, u32))) -> Line {
        let (part_a, part_b) = data;
        Line {
            part_a: part_a.try_into().unwrap(),
            part_b: part_b.try_into().unwrap(),
        }
    }
}

fn solve_part_a(data: &Vec<Line>) -> Option<i32> {
    Some(
        data.into_iter()
            .filter(|line| line.parts_are_subset())
            .count()
            .try_into()
            .unwrap(),
    )
}

fn solve_part_b(data: &Vec<Line>) -> Option<u32> {
    Some(
        data.into_iter()
            .filter(|line| line.parts_overlap())
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn solve(data: String) -> Solution {
    let lines = data.lines().map(Line::from_string).collect::<Vec<Line>>();
    solution!(solve_part_a(&lines), solve_part_b(&lines), 4)
}

fn main() {
    let args = Args::parse();
    match read_input(4, args) {
        Ok(data) => println!("{}", solve(data)),
        Err(e) => eprintln!("Failed to read input file. {}", e),
    };
}