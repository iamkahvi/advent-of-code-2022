fn main() {
    // too much string parsing 😠
    let input: Vec<&str> = include_str!("../input/test.txt").lines().collect();

    dbg!(&input);
}
