use memmap::Mmap;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let file = File::open("./inputs/i06.txt").expect("File not found");
    let mmap = unsafe { Mmap::map(&file).expect("Error while mapping file") };
    let target = 60;
    let mut hm = HashMap::new();
    let mut num = 0;
    for i in 0..(target) {
        let value = &mmap[i];
        match hm.get(value) {
            Some(count) if *count == 0 => {
                num += 1;
                hm.insert(value, count + 1);
            }
            Some(count) => {
                hm.insert(value, count + 1);
            }
            None => {
                num += 1;
                hm.insert(value, 1);
            }
        };
    }
    for i in target..(mmap.len() - target) {
        if i % 1000000 == 0 {
            println!("{i}");
        }
        if num == target {
            println!("{i}");
            break;
        }
        let value = &mmap[i];
        let old_value = &mmap[i - target];
        match hm.get(value) {
            Some(count) if *count == 0 => {
                num += 1;
                hm.insert(value, count + 1);
            }
            Some(count) => {
                hm.insert(value, count + 1);
            }
            None => {
                num += 1;
                hm.insert(value, 1);
            }
        };
        if let Some(count) = hm.get(old_value) {
            let new = count - 1;
            if new == 0 {
                num -= 1;
            }
            hm.insert(value, new);
        };
    }
}
