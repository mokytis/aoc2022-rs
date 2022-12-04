use clap::Parser;
use std::path::PathBuf;

pub struct Solution {
    pub day: usize,
    pub part_a: Option<String>,
    pub part_b: Option<String>,
}

#[macro_export]
macro_rules! solution {
    ($a:expr,$b:expr, $d:expr) => {{
        Solution {
            part_a: match $a {
                Some(a) => Some(a.to_string()),
                None => None,
            },
            part_b: match $b {
                Some(b) => Some(b.to_string()),
                None => None,
            },
            day: $d,
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
        write!(
            f,
            "Day {}:\n  Part A: {}\n  Part B: {}",
            self.day, part_a, part_b,
        )
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false, help = "Use test data file")]
    pub test_data: bool,
    #[arg(short, long, help = "Use test data file")]
    pub input_file: Option<PathBuf>,
}

pub fn read_input(day: usize, args: Args) -> Result<String, std::io::Error> {
    let file = match args.input_file {
        Some(filename) => filename,
        None => PathBuf::from(format!(
            "./inputs/day{:0>2}{}.txt",
            day,
            match args.test_data {
                true => "-test",
                false => "",
            }
        )),
    };
    std::fs::read_to_string(file)
}
