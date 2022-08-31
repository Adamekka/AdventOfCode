use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut floor: i32 = 0;
    let mut floor2 = 0;
    let mut file = File::open("input.txt").expect("Can't open file!");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Can't read the file!");
    
    for (index, i) in input.chars().enumerate() {
        if i == '(' {
            floor += 1;
        } else if i == ')' {
            floor -= 1;
        }
        if floor == -1 {
            floor2 = index + 1;
            break; //to get the 1st answer, remove this line
        }
    }
    println!("{floor}");
    println!("{floor2}");
}
