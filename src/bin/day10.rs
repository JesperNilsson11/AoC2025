use std::{collections::HashSet, fs, path::Path};

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        let indicator = v[0];
        let mut buttons = Vec::new();
        for i in 1..v.len() - 1 {
            let bs = &v[i][1..v[i].len() - 1];
            let button: Vec<usize> = bs.split(",").map(|b| b.parse().unwrap()).collect();
            buttons.push(button);
        }

        let indicator = &indicator[1..indicator.len() - 1];
        let indicators: Vec<bool> = indicator.bytes().map(|b| b == b'#').collect();

        let mut found = false;
        let mut i = 0;
        let mut states = Vec::new();
        states.push(vec![false; indicators.len()]);
        let mut seen = HashSet::new();
        seen.insert(vec![false; indicators.len()]);
        while !found {
            i += 1;
            let mut new_states = Vec::new();
            'outer: for s in states {
                for b in &buttons {
                    let mut new_s = s.clone();
                    for &idx in b {
                        new_s[idx] = !new_s[idx];
                    }

                    if new_s == indicators {
                        found = true;
                        break 'outer;
                    }
                    if !seen.contains(&new_s) {
                        new_states.push(new_s);
                    }
                }
            }

            states = new_states;
        }
        res += i;
    }

    res
}

fn func(
    buttons: &Vec<Vec<usize>>,
    target: &Vec<i32>,
    current_presses: i32,
    mut min_presses: i32,
) -> i32 {
    let mut best = 10000;
    if buttons.is_empty() {
        return best;
    }

    let mut remove = Vec::new();
    for (idx, b) in buttons.iter().enumerate() {
        let mut min = i32::MAX;
        for &ti in b {
            if target[ti] < min {
                min = target[ti];
            }
        }

        if min == 0 {
            remove.push(idx);
        }
    }

    let mut new_buttons = buttons.clone();
    while let Some(r) = remove.pop() {
        new_buttons.remove(r);
    }
    if new_buttons.is_empty() {
        return best;
    }

    let mut contestants = vec![0; new_buttons.len()];
    for (i, &t) in target.iter().enumerate() {
        if t == 0 {
            continue;
        }
        let mut tmp_cnt = Vec::new();
        for (bi, b) in new_buttons.iter().enumerate() {
            for &ti in b {
                if ti == i {
                    tmp_cnt.push(bi);
                    break;
                }
            }
        }

        if contestants.len() > tmp_cnt.len() {
            contestants = tmp_cnt;
        }
    }

    let mut bidx = 0;
    let mut blen = 0;

    if contestants.is_empty() {
        return 10000;
    }
    let mut once = false;
    if contestants.len() == 1 {
        once = true;
    }
    for c in contestants {
        if new_buttons[c].len() > blen {
            blen = new_buttons[c].len();
            bidx = c;
        }
    }

    let button = new_buttons.remove(bidx);
    let mut max = best;
    for &b in &button {
        if target[b] < max {
            max = target[b];
        }
    }
    let mut new_target = target.clone();
    while max >= 0 {
        new_target[..].copy_from_slice(&target[..]);
        let mut done = true;
        let mut min_needed_left = 0;
        for &b in &button {
            new_target[b] -= max;
        }
        for &nt in &new_target {
            if nt > min_needed_left {
                min_needed_left = nt;
            }
            if nt > 0 {
                done = false;
            } else if nt < 0 {
                panic!();
            }
        }
        if done {
            return max;
        }
        if min_needed_left + current_presses >= min_presses {
            return best;
        }

        let tmp = func(
            &new_buttons,
            &new_target,
            current_presses + max,
            min_presses,
        );
        if max + tmp < best {
            best = max + tmp;
        }
        if best + current_presses < min_presses {
            min_presses = best + current_presses;
        }
        max -= 1;

        if once {
            break;
        }
    }

    best
}

fn part2(file: String) -> i32 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        let joltage = *v.last().unwrap();
        let mut buttons = Vec::new();
        let mut buttons_idx = Vec::new();
        for (bidx, i) in (1..v.len() - 1).enumerate() {
            let bs = &v[i][1..v[i].len() - 1];
            let button: Vec<usize> = bs.split(",").map(|b| b.parse().unwrap()).collect();
            buttons_idx.push((button.clone(), bidx));
            buttons.push(button);
        }

        let joltage = &joltage[1..joltage.len() - 1];
        let joltages: Vec<i32> = joltage.split(",").map(|b| b.parse().unwrap()).collect();

        let min = i32::MAX;
        res += func(&buttons, &joltages, 0, min);
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
    assert_eq!(7, part1("test".to_string() + &nr.to_string() + ".txt"));
}

#[test]
fn test_part2() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(33, part2("test".to_string() + &nr.to_string() + ".txt"));
}
