use aoc2022::{solution, Solution, Solver};

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Shape {
    fn shape_value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn better_than(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn worse_than(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

struct Round {
    opponent_shape: Shape,
    your_shape: Shape,
}

impl Round {
    fn from_row_part_a(data: &str) -> Result<Round, &str> {
        let mut data = data.split_whitespace();
        let o = match data.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err("Invalid data format"),
        };
        let y = match data.next().unwrap() {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => return Err("Invalid data format"),
        };

        Ok(Round {
            opponent_shape: o,
            your_shape: y,
        })
    }

    fn from_row_part_b(data: &str) -> Result<Round, &str> {
        let mut data = data.split_whitespace();
        let o = match data.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err("Invalid data format"),
        };
        let y = match data.next().unwrap() {
            "X" => o.better_than(),
            "Y" => o.clone(),
            "Z" => o.worse_than(),
            _ => return Err("Invalid data format"),
        };

        Ok(Round {
            opponent_shape: o,
            your_shape: y,
        })
    }

    fn outcome(&self) -> Outcome {
        if self.your_shape == self.opponent_shape {
            return Outcome::Draw;
        } else if self.your_shape.better_than() == self.opponent_shape {
            return Outcome::Win;
        }
        Outcome::Lose
    }

    fn score(&self) -> i32 {
        self.your_shape.shape_value()
            + match self.outcome() {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Lose => 0,
            }
    }
}

fn parse_data_and_play_games(parser: fn(&str) -> Result<Round, &str>, data: String) -> Option<i32> {
    let rounds: Result<Vec<Round>, &str> = data.lines().map(parser).collect();
    match rounds {
        Ok(r) => Some(r.iter().map(|round| round.score()).sum()),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

fn solve_part_a(data: String) -> Option<i32> {
    parse_data_and_play_games(Round::from_row_part_a, data)
}

fn solve_part_b(data: String) -> Option<i32> {
    parse_data_and_play_games(Round::from_row_part_b, data)
}

pub fn solve(data: String) -> Solution {
    solution!(solve_part_a(data.clone()), solve_part_b(data))
}

pub fn make_solution() -> Solver {
    Solver {
        day: 2,
        solver: solve,
    }
}
