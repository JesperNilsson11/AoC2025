use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res: i64 = 0;

    for bank in input.lines() {
        let mut highest: u8 = 0;
        let bytes = bank.as_bytes();
        let mut idx = 0;
        let mut i = 0;
        for b in &bytes[0..bytes.len() - 1] {
            if *b > highest {
                highest = *b;
                idx = i;
            }
            i += 1;
        }

        res += 10 * (highest - '0' as u8) as i64;
        let mut highest: u8 = 0;
        for b in &bytes[idx + 1..] {
            if *b > highest {
                highest = *b;
            }
        }
        res += (highest - '0' as u8) as i64;
    }

    return res;
}

fn calc(bytes: &[u8], numbers_left: usize) -> (i64, usize) {
    let mut highest: u8 = 0;
    let mut idx = 0;
    let mut i = 0;
    for b in &bytes[0..bytes.len() - numbers_left] {
        if *b > highest {
            highest = *b;
            idx = i;
        }
        i += 1;
    }

    return ((highest - '0' as u8) as i64, idx);
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    for bank in input.lines() {
        let mut tmp = 0;
        let bytes = bank.as_bytes();
        let mut curr_idx = 0;

        for num_left in (0..=11).rev() {
            let (n, i) = calc(&bytes[curr_idx..], num_left);
            tmp *= 10;
            tmp += n;
            curr_idx += i + 1;
        }
        res += tmp;
    }

    return res;
}

fn main() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
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
    assert_eq!(357, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(3121910778619, part2("test".to_string() + &nr.to_string() + ".txt"));
}
