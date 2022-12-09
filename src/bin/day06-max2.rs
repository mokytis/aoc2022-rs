use memmap::Mmap;
use std::fs::File;

fn index_of_value(value: &u8) -> u8 {
    if (65..=90).contains(value) {
        return value - 65;
    }
    if (97..=122).contains(value) {
        return value - 97 + 26;
    }
    value - 48 + 52
}

fn main() {
    let file = File::open("./inputs/i06.txt").expect("File not found");
    let mmap = unsafe { Mmap::map(&file).expect("Error while mapping file") };
    let target = 60;
    let mut values: [u8; 62] = [0; 62];
    let mut num = 0;
    for i in 0..(target) {
        let value = &mmap[i];
        let index = index_of_value(value) as usize;
        if values[index] == 0 {
            num += 1;
        }
        values[index] += 1;
    }
    for i in target..(mmap.len() - target) {
        if num == target {
            println!("{i}");
            break;
        }
        let value = &mmap[i];
        let old_value = &mmap[i - target];
        let index = index_of_value(value) as usize;
        let old_index = index_of_value(old_value) as usize;
        if values[index] == 0 {
            num += 1;
        }
        values[index] += 1;
        if values[old_index] == 1 {
            num -= 1;
        }
        values[old_index] -= 1;
    }
}
