use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut floor: i32 = 0;
    let mut file = File::open("input.txt").expect("Can't open file!");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Can't read the file!");
    
    for i in input.chars() {
        if i == '(' {
            floor += 1;
        } else if i == ')' {
            floor -= 1;
        }
    }
    println!("{floor}");
}
