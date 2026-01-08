use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut iter = input.split("\n\n");
    let ranges: Vec<Vec<i64>> = iter
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split("-").map(|n| n.parse().unwrap()).collect())
        .collect();
    let ids: Vec<i64> = iter
        .next()
        .unwrap()
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    for i in ids {
        let mut found = false;

        for r in &ranges {
            if i >= r[0] && i <= r[1] {
                found = true;
                break;
            }
        }

        if found {
            res += 1;
        }
    }

    res
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut iter = input.split("\n\n");
    let mut ranges: Vec<Vec<i64>> = iter
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split("-").map(|n| n.parse().unwrap()).collect())
        .collect();

    ranges.sort();
    let mut new_range = Vec::new();
    new_range.push(ranges[0].clone());
    for r in ranges {
        let l = new_range.last_mut().unwrap();

        if r[0] < l[0] {
            l[0] = r[0];
        }
        if r[0] <= l[1] && r[1] >= l[1] {
            l[1] = r[1];
        }

        if r[0] > l[1] {
            new_range.push(r);
        }
    }

    for r in new_range {
        res += r[1] - r[0] + 1;
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
    assert_eq!(3, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(14, part2("test".to_string() + &nr.to_string() + ".txt"));
}
