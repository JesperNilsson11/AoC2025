use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    for line in input.lines() {
        for range in line.split(',') {
            let mut iter = range.split('-');
            let start = iter.next().unwrap().parse::<i64>().unwrap();
            let end = iter.next().unwrap().parse::<i64>().unwrap();

            for num in start..=end {
                let strnum = num.to_string();
                let half = strnum.len() / 2;
                if strnum[0..half] == strnum[half..] {
                    res += num;
                }
            }
        }
    }

    res
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    for line in input.lines() {
        for range in line.split(',') {
            let mut iter = range.split('-');
            let start = iter.next().unwrap().parse::<i64>().unwrap();
            let end = iter.next().unwrap().parse::<i64>().unwrap();

            for num in start..=end {
                let strnum = num.to_string();
                let half = strnum.len() / 2;
                for i in 1..=half {
                    if strnum.len() % i != 0 {
                        continue;
                    }
                    let mut matched = true;
                    let pattern = &strnum[0..i];
                    for j in 0..(strnum.len() / i) {
                        if pattern != &strnum[j * i..(j + 1) * i] {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        res += num;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let file = file!();
    let nr = Path::new(file).file_stem().unwrap().to_str().unwrap()[3..]
        .parse::<i32>()
        .unwrap_or(0);

    println!("Day {}!", nr);
    println!("{}", part1("input".to_string() + &nr.to_string() + ".txt"));
    println!("{}", part2("input".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part1() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(
        1227775554,
        part1("test".to_string() + &nr.to_string() + ".txt")
    );
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(
        4174379265,
        part2("test".to_string() + &nr.to_string() + ".txt")
    );
}
