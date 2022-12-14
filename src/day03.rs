use aoc2022::{solution, Solution, Solver};
use std::collections::HashSet;

fn priority(c: char) -> Option<u32> {
    let value: u32 = c.into();
    if c.is_ascii_uppercase() {
        return Some(value - ('A' as u32) + 27);
    } else if c.is_ascii_lowercase() {
        return Some(value - ('a' as u32) + 1);
    }
    None
}

fn find_duplicated_item(bag: &str) -> Option<char> {
    let mut comp_1: HashSet<char> = HashSet::new();
    for (i, c) in bag.chars().enumerate() {
        if i < bag.len() / 2 {
            comp_1.insert(c);
        } else if comp_1.contains(&c) {
            return Some(c);
        }
    }
    None
}

fn solve_part_a(data: String) -> Option<u32> {
    Some(
        data.lines()
            .map(find_duplicated_item)
            .collect::<Option<Vec<char>>>()?
            .into_iter()
            .map(priority)
            .collect::<Option<Vec<u32>>>()?
            .into_iter()
            .sum(),
    )
}

fn solve_part_b(data: String) -> Option<u32> {
    Some(
        data.lines()
            .array_chunks::<3>()
            .map(|lines| {
                let bag_a = lines[0].chars().collect::<HashSet<char>>();
                let bag_b = lines[1]
                    .chars()
                    .filter(|c| bag_a.contains(c))
                    .collect::<HashSet<char>>();
                lines[2].chars().find(|c| bag_b.contains(c))
            })
            .collect::<Option<Vec<char>>>()?
            .into_iter()
            .map(priority)
            .collect::<Option<Vec<u32>>>()?
            .into_iter()
            .sum(),
    )
}

pub fn solve(data: String) -> Solution {
    solution!(solve_part_a(data.clone()), solve_part_b(data))
}

pub fn make_solution() -> Solver {
    Solver {
        day: 3,
        solver: solve,
    }
}
