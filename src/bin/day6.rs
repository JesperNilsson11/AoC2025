use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let map: Vec<Vec<&str>> = input.lines().map(|l| l.split_whitespace().collect()).collect();

    for x in 0..map[0].len() {
        let mut tmp: i64 = 0;
        let op = match map.last().unwrap()[x] {
            "+" => |n1: i64, n2: i64| n1 + n2,
            "*" => {tmp = 1;
                |n1, n2| n1 * n2},
            _ => panic!(),
        };
        for y in 0..map.len()-1 {
            tmp = op(tmp, map[y][x].parse().unwrap());
        }
        res += tmp;
    }

    return res;
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let lines: Vec<_> = input.lines().collect();

    let mut tmp = 0;
    let mut op: fn(i64, i64) -> i64 = |_n1, _n2| panic!();
    for x in 0..lines[0].len() {
        if lines.last().unwrap().as_bytes()[x] == '*' as u8 {
            res += tmp;
            tmp = 1;
            op = |n1, n2| n1 * n2;
        } else if lines.last().unwrap().as_bytes()[x] == '+' as u8 {
            op = |n1, n2| n1 + n2;
        }

        let mut num = 0;
        for y in 0..lines.len()-1 {
            if lines[y].as_bytes()[x] != b' ' {
                num *= 10;
                num += (lines[y].as_bytes()[x] - b'0') as i64;
            }
        }
        if num > 0 {
            tmp = op(tmp, num);
        }
    }
    res += tmp;

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
    assert_eq!(4277556, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..]).parse::<i32>().unwrap_or(0);
    assert_eq!(3263827, part2("test".to_string() + &nr.to_string() + ".txt"));
}