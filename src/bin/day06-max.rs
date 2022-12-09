use std::collections::HashSet;
use std::path::PathBuf;

fn main() {
    println!(
        "{}",
        std::fs::read_to_string(PathBuf::from("./inputs/i06.txt"))
            .unwrap()
            .as_bytes()
            .windows(60)
            .enumerate()
            .find(|(_, slice)| {
                let set = slice.iter().collect::<HashSet<&u8>>();
                set.len() == 60
            })
            .unwrap()
            .0
            + 60
    );
}
