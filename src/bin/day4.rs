use std::{fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != b'@' {
                continue;
            }

            let mut neighbours = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let xx: i32 = x as i32 + dx;
                    let yy: i32 = y as i32 + dy;

                    if xx < 0 || yy < 0 || xx >= map[y].len() as i32 || yy >= map.len() as i32 {
                        continue;
                    }
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if map[yy as usize][xx as usize] == b'@' {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                res += 1;
            }
        }
    }

    res
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect();

    let mut removed = true;
    while removed {
        removed = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != b'@' {
                    continue;
                }

                let mut neighbours = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let xx: i32 = x as i32 + dx;
                        let yy: i32 = y as i32 + dy;

                        if xx < 0 || yy < 0 || xx >= map[y].len() as i32 || yy >= map.len() as i32 {
                            continue;
                        }
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if map[yy as usize][xx as usize] == b'@' {
                            neighbours += 1;
                        }
                    }
                }
                if neighbours < 4 {
                    removed = true;
                    map[y][x] = b'x';
                    res += 1;
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
    assert_eq!(13, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(43, part2("test".to_string() + &nr.to_string() + ".txt"));
}
