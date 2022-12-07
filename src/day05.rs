use aoc2022::{solution, Solution, Solver};

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new_empty() -> Stack {
        Stack { crates: vec![] }
    }
}

struct Move {
    amount: usize,
    source: usize,
    destination: usize,
}

impl Move {
    fn from_line(line: &str) -> Option<Move> {
        let mut chars = line.chars();
        let amount = chars
            .by_ref()
            .skip(5)
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .ok()?;
        let source = chars
            .by_ref()
            .skip(5)
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .ok()?;
        let destination = chars
            .skip(3)
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .ok()?;

        Some(Move {
            amount,
            source: source - 1,
            destination: destination - 1,
        })
    }
}

fn solve_part_a(mut stacks: Vec<Stack>, moves: &Vec<Move>) -> Option<String> {
    for m in moves {
        for _ in 0..m.amount {
            let value = stacks[m.source].crates.pop()?;
            stacks[m.destination].crates.push(value)
        }
    }

    Some(
        stacks
            .into_iter()
            .map(|s| s.crates[s.crates.len() - 1])
            .collect::<String>(),
    )
}

fn solve_part_b(mut stacks: Vec<Stack>, moves: &Vec<Move>) -> Option<String> {
    for m in moves {
        let mut temp: Vec<char> = vec![];
        for _ in 0..m.amount {
            let value = stacks[m.source].crates.pop()?;
            temp.push(value);
        }
        while !temp.is_empty() {
            stacks[m.destination].crates.push(temp.pop()?);
        }
    }

    Some(
        stacks
            .into_iter()
            .map(|s| s.crates[s.crates.len() - 1])
            .collect::<String>(),
    )
}

pub fn solve(data: String) -> Solution {
    let mut stacks: Vec<Stack> = vec![];

    let mut lines = data.lines();

    lines
        .by_ref()
        .take_while(|line| line.contains('['))
        .for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphabetic())
                .for_each(|(i, c)| {
                    while stacks.len() <= i {
                        stacks.push(Stack::new_empty())
                    }
                    stacks[i].crates.insert(0, c)
                })
        });

    let moves = lines
        .by_ref()
        .skip(1)
        .map(Move::from_line)
        .collect::<Option<Vec<Move>>>()
        .unwrap();

    solution!(
        solve_part_a(stacks.clone(), &moves),
        solve_part_b(stacks, &moves),
        5
    )
}

pub fn make_solution() -> Solver {
    Solver {
        day: 5,
        solver: solve,
    }
}
