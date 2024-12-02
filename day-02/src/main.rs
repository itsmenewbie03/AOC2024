fn map_fn(levels: Vec<i32>) -> bool {
    let inc = levels[0] - levels[1] < 0;
    if inc {
        let mut lvl_clone = levels.clone();
        lvl_clone.sort();
        if levels != lvl_clone {
            return false;
        }
    } else {
        let mut lvl_clone = levels.clone();
        lvl_clone.sort();
        lvl_clone.reverse();
        if levels != lvl_clone {
            return false;
        }
    };
    levels
        .windows(2)
        .all(|pair| (pair[0] - pair[1]).abs() > 0 && (pair[0] - pair[1]).abs() <= 3)
}
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");
    let answer: i32 = input
        .trim()
        .split("\n")
        .map(|x| {
            let mut inconsistent = false;
            let levels: Vec<i32> = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("failed to convert to int"))
                .collect();
            let inc = levels[0] - levels[1] < 0;
            if inc {
                let mut lvl_clone = levels.clone();
                lvl_clone.sort();
                if levels != lvl_clone {
                    inconsistent = true;
                }
            } else {
                let mut lvl_clone = levels.clone();
                lvl_clone.sort();
                lvl_clone.reverse();
                if levels != lvl_clone {
                    inconsistent = true;
                }
            }
            if !inconsistent {
                let passed = levels
                    .windows(2)
                    .all(|pair| (pair[0] - pair[1]).abs() > 0 && (pair[0] - pair[1]).abs() <= 3);
                if passed {
                    return true;
                }
            }
            // NOTE: time to use problem damper
            // this won't be pretty implementation xD
            for i in 0..levels.len() {
                let level_var: Vec<i32> = levels
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i)
                    .map(|(_, elem)| *elem)
                    .collect();
                if map_fn(level_var) {
                    return true;
                }
            }
            false
        })
        .filter(|x| *x)
        .count() as i32;
    println!("That was tricky but I guess I did it {answer}");
}
