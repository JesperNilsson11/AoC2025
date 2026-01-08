use std::{collections::HashMap, fs, path::Path};

fn dfs(curr_server: &str, map: &HashMap<&str, Vec<&str>>) -> i64 {
    if curr_server == "out" {
        return 1;
    }

    let mut ret = 0;
    for &new in map.get(curr_server).unwrap() {
        ret += dfs(new, map);
    }

    ret
}

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();

    let mut map = HashMap::new();
    for l in input.lines() {
        let mut itr = l.split_whitespace();
        let key = itr.next().unwrap();
        let key = &key[..key.len() - 1];
        map.insert(key, Vec::new());

        for s in itr {
            map.get_mut(key).unwrap().push(s);
        }
    }

    dfs("you", &map)
}

fn dfs2(
    curr_server: String,
    map: &HashMap<String, Vec<String>>,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(String, bool, bool), i64>,
) -> i64 {
    if cache.contains_key(&(curr_server.clone(), dac, fft)) {
        return *cache.get(&(curr_server.clone(), dac, fft)).unwrap();
    }
    if curr_server == "out" {
        if dac && fft {
            return 1;
        }
        return 0;
    }

    let mut ret = 0;
    for new in map.get(&curr_server).unwrap() {
        ret += dfs2(
            new.clone(),
            map,
            dac || new == "dac",
            fft || new == "fft",
            cache,
        );
    }

    cache.insert((curr_server, dac, fft), ret);

    ret
}

fn part2(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();

    let mut map = HashMap::new();
    for l in input.lines() {
        let mut itr = l.split_whitespace();
        let key = itr.next().unwrap();
        let key = &key[..key.len() - 1];
        let key = key.to_string();
        map.insert(key.clone(), Vec::new());

        for s in itr {
            map.get_mut(&key).unwrap().push(s.to_string());
        }
    }

    let mut cache = HashMap::new();

    dfs2("svr".to_string(), &map, false, false, &mut cache)
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
