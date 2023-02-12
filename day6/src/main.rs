use std::{collections::HashSet, slice::Windows};

const WINDOW_SIZE: usize = 14;

fn main() {
    let lines = include_str!("../input/input.txt")
        .lines()
        .collect::<Vec<&str>>();

    for line in lines {
        dbg!(solve_line(line).unwrap());
    }
}

fn solve_line(line: &str) -> Result<usize, &str> {
    let chars = line.chars().collect::<Vec<char>>();
    let windows: Windows<char> = chars.windows(WINDOW_SIZE);

    'outer: for (i, w) in windows.enumerate() {
        let mut h = HashSet::<&char>::new();
        for (j, c) in w.iter().enumerate() {
            if h.contains(c) {
                continue 'outer;
            }
            h.insert(c);
            if j == (w.len() - 1) {
                return Ok(i + WINDOW_SIZE);
            }
        }
    }

    Err("nope")
}
