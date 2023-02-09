use std::fs;

#[derive(Debug)]
struct Rucksack {
    first: Vec<Item>,
    second: Vec<Item>,
}

#[derive(Debug)]

struct Er(&'static str);

#[derive(Debug, PartialEq, Clone, Copy)]
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

    let matches: Vec<Item> = sacks
        .iter()
        .filter_map(|s| {
            for c in &s.first {
                if s.second.contains(&c) {
                    return Some(c.clone());
                }
            }
            return None;
        })
        .collect();

    let match_vals: Vec<usize> = matches.iter().map(|i| i.score()).collect();

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

            let (first, second) = l.split_at(len / 2);

            Some(Rucksack {
                first: first.chars().map(|c| Item(c as u8)).collect(),
                second: second.chars().map(|c| Item(c as u8)).collect(),
            })
        })
        .collect();

    Ok(r)
}
