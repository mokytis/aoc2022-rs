use clap::Parser;
use std::path::PathBuf;

pub struct Solution {
    pub part_a: Option<String>,
    pub part_b: Option<String>,
}

pub struct Solver {
    pub day: usize,
    pub solver: fn(String) -> Solution,
}

impl Solver {
    pub fn solve(&self, test_data: bool) {
        println!("= Day {} =", self.day);
        match read_input(self.day, test_data) {
            Ok(data) => println!("{}", (self.solver)(data)),
            Err(e) => eprintln!("Failed to read input file. {e}"),
        };
        println!()
    }
}

#[macro_export]
macro_rules! solution {
    ($a:expr,$b:expr) => {{
        Solution {
            part_a: match $a {
                Some(a) => Some(a.to_string()),
                None => None,
            },
            part_b: match $b {
                Some(b) => Some(b.to_string()),
                None => None,
            },
        }
    }};
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part_a = match &self.part_a {
            Some(value) => value,
            None => "Unsolved",
        };
        let part_b = match &self.part_b {
            Some(value) => value,
            None => "Unsolved",
        };
        write!(f, "Part A: {part_a}\nPart B: {part_b}")
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false, help = "Use test data file")]
    pub test_data: bool,
    #[arg(short, long, help = "Use test data file")]
    pub input_file: Option<PathBuf>,
    #[arg(short, long, help = "Just run one day")]
    pub day: Option<usize>,
}

pub fn read_input(day: usize, use_test_data: bool) -> Result<String, std::io::Error> {
    let file = PathBuf::from(format!(
        "./inputs/day{:0>2}{}.txt",
        day,
        match use_test_data {
            true => "-test",
            false => "",
        }
    ));
    std::fs::read_to_string(file)
}
