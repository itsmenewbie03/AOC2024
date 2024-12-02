use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .trim()
        .split("\n")
        .map(|x| {
            let temp = x.split_whitespace().collect::<Vec<&str>>();
            (
                temp[0].parse::<i32>().unwrap(),
                temp[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    let p1_answer: i32 = zip(&left, &right).map(|(l, r)| (l - r).abs()).sum();
    println!("Did we do it: {p1_answer}");
    // INFO: part 2
    let p2_answer: i32 = right
        .iter()
        .enumerate()
        .map(|(idx, x)| x * left.iter().filter(|y| *x == **y).count() as i32)
        .sum();
    println!("Did we do it first try: {p2_answer}");
}
