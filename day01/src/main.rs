use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn solve<T>(input: T) -> (u32, u32)
where
    T: Iterator<Item = String>,
{
    let (max_cal, max_idx, _, _) = input.fold(
        (0, 1, 0, 1),
        |(max_cal, max_idx, cur_cal, cur_idx), line| {
            if line.len() == 0 {
                if cur_cal > max_cal {
                    (cur_cal, cur_idx, 0, cur_idx + 1)
                } else {
                    (max_cal, max_idx, 0, cur_idx + 1)
                }
            } else {
                let value: u32 = FromStr::from_str(&line).expect("Could not parse integer.");
                (max_cal, max_idx, cur_cal + value, cur_idx)
            }
        },
    );
    (max_cal, max_idx)
}

fn solve_2<T>(input: T) -> u32
where
    T: Iterator<Item = String>,
{
    let mut carrying: Vec<u32> = Vec::new();
    let mut cur_cal = 0;
    for l in input {
        if l.len() == 0 {
            carrying.push(cur_cal);
            cur_cal = 0;
        } else {
            let val: u32 = FromStr::from_str(&l).expect("Failed to parse integer.");
            cur_cal += val
        }
    }
    carrying.sort_by(|a, b| b.cmp(a));
    dbg!(&carrying);
    carrying[0..3].iter().fold(0, |a, b| a + b)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fileinput = File::open(&args[1]).expect("Failed to open file.");
    let reader = BufReader::new(fileinput);
    // let (cal, elf) = solve(reader.lines().map(|r| r.expect("Failed to read line.")));
    // println!(
    //     "Elf {} carries {} calories which is the most any elf carries.",
    //     elf, cal
    // );
    println!(
        "Top three elves are carrying {} calories.",
        solve_2(reader.lines().map(|r| r.expect("Failed to read line.")))
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let str_input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;
        let (cal, idx) = solve(str_input.lines().map(|s| s.to_string()));
        assert_eq!(cal, 24000);
        assert_eq!(idx, 4);
    }

    #[test]
    fn example_2() {
        let str_input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;
        let result = solve_2(str_input.lines().map(|s| s.to_string()));
        assert_eq!(result, 45000);
    }
}
