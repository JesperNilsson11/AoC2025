use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let mut pos = (0, 0);
    'outer: for (y, r) in map.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if c == b'S' {
                pos = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    let mut beams = HashSet::new();
    beams.insert(pos);

    while !beams.is_empty() {
        let mut new_beams = HashSet::new();

        for b in beams.iter() {
            let x = b.0;
            let mut y = b.1;
            y += 1;
            if y >= map.len() as i32 {
                continue;
            }
            if map[y as usize][x as usize] == b'^' {
                let x1 = x - 1;
                let x2 = x + 1;
                res += 1;

                if x1 >= 0 && x1 < map[0].len() as i32 {
                    new_beams.insert((x1, y));
                }
                if x2 >= 0 && x2 < map[0].len() as i32 {
                    new_beams.insert((x2, y));
                }
            } else {
                new_beams.insert((x, y));
            }
        }

        beams = new_beams;
    }

    res
}

fn dfs(x: i64, yy: i64, map: &Vec<Vec<u8>>, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut res = 0;

    let mut y = yy;
    if memo.contains_key(&(x, y)) {
        return *memo.get(&(x, y)).unwrap();
    }

    y += 1;
    if y >= map.len() as i64 {
        return 1;
    }

    if map[y as usize][x as usize] == b'.' {
        while y + 1 < map.len() as i64 && map[(y + 1) as usize][x as usize] == b'.' {
            y += 1;
        }
        y += 1;
    }

    if y == map.len() as i64 {
        return 1;
    }

    let x1 = x - 1;
    let x2 = x + 1;
    assert!(x1 >= 0);
    assert!(x2 <= map[0].len() as i64);

    res += dfs(x1, y, map, memo);
    res += dfs(x2, y, map, memo);

    if memo.insert((x, yy), res).is_some() {
        panic!();
    }

    res
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let mut pos = (0, 0);
    'outer: for (y, r) in map.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if c == b'S' {
                pos = (x as i64, y as i64);
                break 'outer;
            }
        }
    }
    let mut memo = HashMap::new();

    res += dfs(pos.0, pos.1, &map, &mut memo);

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
    assert_eq!(21, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(40, part2("test".to_string() + &nr.to_string() + ".txt"));
}
