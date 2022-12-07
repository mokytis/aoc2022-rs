use memmap::Mmap;
use std::collections::HashSet;
use std::fs::File;

fn main() {
    let file = File::open("./inputs/i06.txt").expect("failed to open the file");

    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    println!(
        "{}",
        mmap.windows(60)
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
