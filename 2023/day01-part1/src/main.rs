use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn first_number(s: String) -> Option<char> {
    for c in s.chars() {
        if '0' <= c && c <= '9' {
            return Some(c);
        }
    }
    None
}

fn last_number(s: String) -> Option<char> {
    for c in s.chars().rev()  {
        if '0' <= c && c <= '9' {
            return Some(c);
        }
    }
    None
}

fn main() {
    let path = Path::new("data/input");

    let f = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("failed to open {}, {}", path.display(), why.to_string()),
    };

    let reader = BufReader::new(f);
    let mut sum: i32 = 0;

    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        let n10: i32;
        let n1: i32;
        match first_number(unwraped_line.clone()) {
            Some(n) => n10 = n as i32 - '0' as i32,
            None => panic!("not found first number"),
        };
        match last_number(unwraped_line.clone()) {
            Some(n) => n1 = n as i32 - '0' as i32,
            None => panic!("not found last number"),
        };

        sum += 10 * n10 + n1;

    }
    println!("{}", sum);
}
