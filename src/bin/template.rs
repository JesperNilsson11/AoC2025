use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();

    0
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();

    0
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
    assert_eq!(0, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(0, part2("test".to_string() + &nr.to_string() + ".txt"));
}
