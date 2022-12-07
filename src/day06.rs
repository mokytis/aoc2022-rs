use aoc2022::{solution, Solution, Solver};
use std::collections::HashSet;

fn find_unique_sliding_window(data: &str, size: usize) -> Option<usize> {
    Some(
        data.as_bytes()
            .windows(size)
            .enumerate()
            .find(|(_, slice)| {
                let set = slice.iter().collect::<HashSet<&u8>>();
                set.len() == size
            })?
            .0
            + size,
    )
}

fn solve_part_a(data: &str) -> Option<usize> {
    find_unique_sliding_window(data, 4)
}

fn solve_part_b(data: &str) -> Option<usize> {
    find_unique_sliding_window(data, 14)
}

pub fn solve(data: String) -> Solution {
    solution!(solve_part_a(&data), solve_part_b(&data), 5)
}

pub fn make_solution() -> Solver {
    Solver {
        day: 6,
        solver: solve,
    }
}
