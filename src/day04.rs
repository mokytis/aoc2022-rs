use aoc2022::{solution, Solution, Solver};

fn parse_group(group: &str) -> (u32, u32) {
    group
        .split_once('-')
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
            .split_once(',')
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
        Range { start, end }
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

fn solve_part_a(data: &[Line]) -> Option<i32> {
    Some(
        data.iter()
            .filter(|line| line.parts_are_subset())
            .count()
            .try_into()
            .unwrap(),
    )
}

fn solve_part_b(data: &[Line]) -> Option<u32> {
    Some(
        data.iter()
            .filter(|line| line.parts_overlap())
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn solve(data: String) -> Solution {
    let lines = data.lines().map(Line::from_string).collect::<Vec<Line>>();
    solution!(solve_part_a(&lines), solve_part_b(&lines))
}

pub fn make_solution() -> Solver {
    Solver {
        day: 4,
        solver: solve,
    }
}
