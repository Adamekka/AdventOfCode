use std::{fs, str::FromStr};

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("no input.txt");
    let mut super_vec: Vec<Vec<i32>> = Vec::new();

    for line in text.lines() {
        let mut line_contents: Vec<i32> = Vec::new();

        if line.is_empty() {
            super_vec.push(line_contents);
            // break;
            // line_contents.push(0);
        } else {
            let line: i32 = FromStr::from_str(line).expect("bruh");
            line_contents.push(line);
        }
    }

    println!("{:?}", super_vec);
}
