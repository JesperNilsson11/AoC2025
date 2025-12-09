use std::{fs, i64, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut grid: Vec<(i64, i64)> = input.lines().map(|l| {
        let mut itr = l.split(',');
        (itr.next().unwrap().parse().unwrap(), itr.next().unwrap().parse().unwrap())
    }).collect();

    for &(x1, y1) in &grid {
        for &(x2, y2) in &grid {
            let a = (1+(x1-x2).abs()) * (1+(y1-y2).abs());
            if res < a {
                res = a;
            }
        }
    }

    return res;
}

fn is_valid(xmin: i64, xmax: i64, ymin: i64, ymax: i64, lines: &Vec<(i64, i64, i64, i64)>) -> bool {
    for l in lines {
        if l.0 >= xmax || l.1 <= xmin || l.3 <= ymin || l.2 >= ymax {
            continue;
        }
        return false;
    }
    return true;
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut points: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let mut itr = l.split(",");
            (
                itr.next().unwrap().parse().unwrap(),
                itr.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut lines = Vec::new();
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        let (xmin, xmax) = if p1.0 > p2.0 {
            (p2.0, p1.0)
        } else {
            (p1.0, p2.0)
        };
        let (ymin, ymax) = if p1.1 > p2.1 {
            (p2.1, p1.1)
        } else {
            (p1.1, p2.1)
        };

        lines.push((xmin, xmax, ymin, ymax));
    }

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        for j in i + 1..points.len() {
            let p2 = points[j];

            let (xmin, xmax) = if p1.0 > p2.0 {
                (p2.0, p1.0)
            } else {
                (p1.0, p2.0)
            };
            let (ymin, ymax) = if p1.1 > p2.1 {
                (p2.1, p1.1)
            } else {
                (p1.1, p2.1)
            };

            if is_valid(xmin, xmax, ymin, ymax, &lines) {
                let area = (xmax-xmin+1) * (ymax-ymin+1);
                if area > res {
                    res = area;
                }
            }
        }
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
    assert_eq!(0, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(24, part2("test".to_string() + &nr.to_string() + ".txt"));
}
