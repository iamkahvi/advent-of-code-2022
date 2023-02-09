struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        for i in r.start..=r.end {
            if !self.contains_num(i) {
                return false;
            }
        }
        true
    }

    fn contains_num(&self, i: usize) -> bool {
        i >= self.start && i <= self.end
    }

    fn overlaps(&self, r: &Range) -> bool {
        for i in r.start..=r.end {
            if self.contains_num(i) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input/input.txt").lines().collect();

    dbg!(&input);

    let res = input
        .iter()
        .filter(|line| {
            let ranges: Vec<usize> = line
                .split(&['-', ','][..])
                .map(|e| e.parse().unwrap())
                .collect();

            match ranges[..] {
                [s1, e1, s2, e2] => {
                    let r1 = Range { start: s1, end: e1 };
                    let r2 = Range { start: s2, end: e2 };

                    r1.contains(&r2) || r2.contains(&r1)
                }
                _ => false,
            }
        })
        .collect::<Vec<&&str>>()
        .len();

    let r1 = Range { start: 3, end: 7 };
    let r2 = Range { start: 2, end: 3 };
    let r3 = Range { start: 3, end: 6 };

    dbg!(r1.contains(&r2));
    dbg!(r1.contains(&r3));

    dbg!(res);

    let res2 = part_2(input.clone());
    dbg!(res2);
}

fn part_2(lines: Vec<&str>) -> usize {
    lines
        .iter()
        .filter(|line| {
            let ranges: Vec<usize> = line
                .split(&['-', ','][..])
                .map(|e| e.parse().unwrap())
                .collect();

            match ranges[..] {
                [s1, e1, s2, e2] => {
                    let r1 = Range { start: s1, end: e1 };
                    let r2 = Range { start: s2, end: e2 };

                    r1.overlaps(&r2) || r2.overlaps(&r1)
                }
                _ => false,
            }
        })
        .collect::<Vec<&&str>>()
        .len()
}
