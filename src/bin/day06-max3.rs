use memmap::Mmap;
use std::fs::File;

fn main() {
    let file = File::open("./inputs/i06.txt").expect("File not found");
    let mmap = unsafe { Mmap::map(&file).expect("Error while mapping file") };
    let target = 60;
    let mut values: [u8; 256] = [0; 256];
    let mut num = 0;

    (0..target).for_each(|i| {
        let value = mmap[i] as usize;
        if values[value] == 0 {
            num += 1;
        }
        values[value] += 1;
    });

    let (_, ans) = (target..(mmap.len() - target))
        .map(|i| {
            let value = mmap[i] as usize;
            let old_value = mmap[i - target] as usize;
            if values[value] == 0 {
                num += 1;
            }
            values[value] += 1;
            if values[old_value] == 1 {
                num -= 1;
            }
            values[old_value] -= 1;

            (num, i)
        })
        .find(|(n, _)| *n == target)
        .unwrap();
    println!("{}", ans + 1);
}
