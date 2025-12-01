use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut pos = 50;

    for line in input.lines() {
        let c = line.chars().next().unwrap();
        let num = line[1..].parse::<i32>().unwrap();
        match c {
            'R' => pos += num,
            'L' => pos -= num,
            _ => (),
        }

        while pos < 0 {
            pos += 100;
        }
        pos = pos % 100;

        if pos == 0 {
            res += 1;
        }
    }

    return res;
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut pos = 50;

    for line in input.lines() {
        let c = line.chars().next().unwrap();
        let mut num = line[1..].parse::<i32>().unwrap();
        let op = match c {
            'R' => |n: &mut i32| { *n += 1 },
            'L' => |n: &mut i32| { *n -= 1 },
            _ => panic!("Invalid direction"),
        };

        while num > 0 {
            op(&mut pos);
            num -= 1;
            if pos % 100 == 0 {
                res += 1;
            }
        }
    }

    return res;
}

fn main() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..]).parse::<i32>().unwrap_or(0);

    println!("Day {}!", nr);
    println!("{}", part1("input".to_string() + &nr.to_string() + ".txt"));
    println!("{}", part2("input".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part1() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..]).parse::<i32>().unwrap_or(0);
    assert_eq!(3, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..]).parse::<i32>().unwrap_or(0);
    assert_eq!(6, part2("test".to_string() + &nr.to_string() + ".txt"));
}