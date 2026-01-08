use std::{collections::HashSet, fs, path::Path};

fn part1(file: String, iterations: usize) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res;

    let mut junctions = Vec::new();

    for line in input.lines() {
        let mut itr = line.split(',');

        let x = itr.next().unwrap().parse::<i64>().unwrap();
        let y = itr.next().unwrap().parse::<i64>().unwrap();
        let z = itr.next().unwrap().parse::<i64>().unwrap();
        junctions.push((x, y, z));
    }

    let mut dists = Vec::new();
    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            let j1 = junctions[i];
            let j2 = junctions[j];
            let dx = j1.0 - j2.0;
            let dy = j1.1 - j2.1;
            let dz = j1.2 - j2.2;
            let d = dx * dx + dy * dy + dz * dz;
            dists.push((d, i, j));
        }
    }
    dists.sort_by_key(|k| k.0);

    let mut circuits = Vec::new();
    for i in 0..junctions.len() {
        let mut c = HashSet::new();
        c.insert(i);
        circuits.push(c);
    }

    for i in 0..iterations {
        let next = dists[i];
        let mut c1 = usize::MAX;
        let mut c2 = usize::MAX;
        for (idx, c) in circuits.iter().enumerate() {
            if c.contains(&next.1) {
                c1 = idx;
            }
            if c.contains(&next.2) {
                c2 = idx;
            }

            if c1 != usize::MAX && c2 != usize::MAX {
                break;
            }
        }

        if c1 != c2 {
            if c2 < c1 {
                std::mem::swap(&mut c2, &mut c1);
            }

            let old = circuits.remove(c2);
            circuits[c1].extend(old);
        }
    }

    let mut lens = Vec::new();
    for c in circuits {
        lens.push(c.len());
    }
    lens.sort();
    res = 1;
    res *= lens.pop().unwrap() as i64;
    res *= lens.pop().unwrap() as i64;
    res *= lens.pop().unwrap() as i64;

    res
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let res;

    let mut junctions = Vec::new();

    for line in input.lines() {
        let mut itr = line.split(',');

        let x = itr.next().unwrap().parse::<i64>().unwrap();
        let y = itr.next().unwrap().parse::<i64>().unwrap();
        let z = itr.next().unwrap().parse::<i64>().unwrap();
        junctions.push((x, y, z));
    }

    let mut dists = Vec::new();
    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            let j1 = junctions[i];
            let j2 = junctions[j];
            let dx = j1.0 - j2.0;
            let dy = j1.1 - j2.1;
            let dz = j1.2 - j2.2;
            let d = dx * dx + dy * dy + dz * dz;
            dists.push((d, i, j));
        }
    }
    dists.sort_by_key(|k| k.0);

    let mut circuits = Vec::new();
    for i in 0..junctions.len() {
        let mut c = HashSet::new();
        c.insert(i);
        circuits.push(c);
    }

    let mut i = 0;
    loop {
        let next = dists[i];
        i += 1;

        let mut c1 = usize::MAX;
        let mut c2 = usize::MAX;
        for (idx, c) in circuits.iter().enumerate() {
            if c.contains(&next.1) {
                c1 = idx;
            }
            if c.contains(&next.2) {
                c2 = idx;
            }

            if c1 != usize::MAX && c2 != usize::MAX {
                break;
            }
        }

        if c1 != c2 {
            if c2 < c1 {
                std::mem::swap(&mut c2, &mut c1);
            }

            if circuits.len() == 2 {
                res = junctions[next.1].0 * junctions[next.2].0;
                break;
            }
            let old = circuits.remove(c2);
            circuits[c1].extend(old);
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
    println!(
        "{}",
        part1("input".to_string() + &nr.to_string() + ".txt", 1000)
    );
    println!("{}", part2("input".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part1() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(40, part1("test".to_string() + &nr.to_string() + ".txt", 10));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(25272, part2("test".to_string() + &nr.to_string() + ".txt"));
}
