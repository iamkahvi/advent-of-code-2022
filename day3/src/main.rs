use std::fs;

#[derive(Debug)]
struct Rucksack {
    first: Vec<char>,
    second: Vec<char>,
}

#[derive(Debug)]

struct Er(&'static str);

#[derive(Debug)]

struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = Er;

    fn try_from(value: u8) -> Result<Self, Er> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(Er("not a valid item")),
        }
    }
}

impl Item {
    fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input_str = fs::read_to_string("./input/input.txt").unwrap();
    let lines = input_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let sacks = parse_lines(lines).unwrap();

    let matches: Vec<char> = sacks
        .iter()
        .filter_map(|s| {
            for c in s.first.clone() {
                if s.second.contains(&c) {
                    return Some(c);
                }
            }
            return None;
        })
        .collect();

    let match_vals: Vec<usize> = matches.iter().map(|m| Item(*m as u8).score()).collect();

    println!("matches: {:?}", matches);
    println!(
        "match vals: {:?}",
        match_vals.iter().fold(0, |acc, x| acc + x)
    );
}

fn parse_lines(lines: Vec<&str>) -> Result<Vec<Rucksack>, Er> {
    let r = lines
        .iter()
        .filter_map(|l| {
            let len = l.len();

            if len % 2 != 0 {
                return None;
            }

            Some(Rucksack {
                first: l[..(len / 2)].chars().collect(),
                second: l[(len / 2)..].chars().collect(),
            })
        })
        .collect();

    Ok(r)
}
