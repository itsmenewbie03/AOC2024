fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut do_mul = true;
    let answer: i32 = input
        .trim()
        .bytes()
        .collect::<Vec<u8>>()
        .windows(4)
        .enumerate()
        .map(|(idx, x)| {
            if x == "don'".as_bytes() {
                let start = idx + 4;
                if &input[start..start + 3] == "t()" {
                    do_mul = false;
                }
            }
            if x == "do()".as_bytes() {
                do_mul = true;
            }
            if x == "mul(".as_bytes() {
                // INFO: we found instance of 'mul('
                // TODO: grab chars until next instance of ','
                let start = idx + 4;
                let sep_idx = input[start..].find(',').expect("can't find a sep") + start;
                let a = &input[start..sep_idx];
                // INFO: grab from sep til non int
                let c_paren_idx = input[sep_idx + 1..]
                    .bytes()
                    .position(|x| !x.is_ascii_digit())
                    .expect("can't find any non ascii digit");
                let c_char = &input[sep_idx + c_paren_idx + 1..sep_idx + c_paren_idx + 2];
                let b = &input[sep_idx + 1..(sep_idx + 1 + c_paren_idx)];
                if !do_mul {
                    return 0;
                }
                if c_char == ")" {
                    a.parse::<i32>().unwrap_or(0) * b.parse::<i32>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    println!("I did it: {answer}");
}
