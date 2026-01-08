use std::{collections::HashSet, fs, path::Path};

fn rotate(s: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut res = vec![vec![false; s.len()]; s[0].len()];

    for y in 0..s.len() {
        for x in 0..s[0].len() {
            res[y][x] = s[2 - x][y];
        }
    }

    res
}

fn flip(s: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut res = s.clone();

    for y in 0..s.len() {
        for x in 0..s[0].len() {
            res[y][x] = s[s.len() - 1 - y][x];
        }
    }

    res
}

fn can_place(shape: &Vec<Vec<bool>>, grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if y + shape.len() > grid.len() {
        return false;
    }
    if x + shape[0].len() > grid[0].len() {
        return false;
    }

    for yy in 0..shape.len() {
        for xx in 0..shape[0].len() {
            if shape[yy][xx] && grid[y + yy][x + xx] {
                return false;
            }
        }
    }

    true
}

fn place(shape: &Vec<Vec<bool>>, grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    for yy in 0..shape.len() {
        for xx in 0..shape[0].len() {
            grid[y + yy][x + xx] = shape[yy][xx];
        }
    }
}

fn dfs(shapes: &Vec<HashSet<Vec<Vec<bool>>>>, shape_nums: &Vec<i64>, grid: Vec<Vec<bool>>) -> bool {
    let mut shape_idx = usize::MAX;
    for (idx, &sn) in shape_nums.iter().enumerate() {
        if sn > 0 {
            shape_idx = idx;
            break;
        }
    }
    if shape_idx == usize::MAX {
        return true;
    }

    let mut new_shape_nums = shape_nums.clone();
    new_shape_nums[shape_idx] -= 1;

    let shape_set = &shapes[shape_idx];
    for shape in shape_set {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if can_place(shape, &grid, x, y) {
                    let mut new_grid = grid.clone();
                    place(shape, &mut new_grid, x, y);

                    if dfs(shapes, &new_shape_nums, new_grid) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn top_dfs(
    shapes: &Vec<HashSet<Vec<Vec<bool>>>>,
    shape_nums: &Vec<i64>,
    grid: Vec<Vec<bool>>,
) -> bool {
    let mut shape_idx = usize::MAX;
    for (idx, &sn) in shape_nums.iter().enumerate() {
        if sn > 0 {
            shape_idx = idx;
            break;
        }
    }
    if shape_idx == usize::MAX {
        return true;
    }

    let mut new_shape_nums = shape_nums.clone();
    new_shape_nums[shape_idx] -= 1;

    let shape_set = &shapes[shape_idx];
    for shape in shape_set {
        if can_place(shape, &grid, 0, 0) {
            let mut new_grid = grid.clone();
            place(shape, &mut new_grid, 0, 0);

            if dfs(shapes, &new_shape_nums, new_grid) {
                return true;
            }
        }
    }

    false
}

fn part1(file: String) -> i64 {
    let input = fs::read_to_string(file).unwrap();
    let mut res = 0;

    let mut groups: Vec<_> = input.split("\n\n").collect();

    let regions = groups.pop().unwrap();

    let mut shapes = Vec::new();
    for g in groups {
        let mut itr = g.trim().lines();

        itr.next();
        let shape: Vec<Vec<bool>> = itr
            .map(|l| l.bytes().map(|b| b == b'#').collect())
            .collect();

        shapes.push(shape);
    }

    let mut fr_shapes = Vec::new();
    let mut shape_sizes = Vec::new();
    for s in shapes {
        let mut set = HashSet::new();
        let ns = rotate(&s);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());

        let ns = flip(&ns);

        let ns = rotate(&ns);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());
        let ns = rotate(&ns);
        set.insert(ns.clone());

        fr_shapes.push(set);
        let mut size = 0;
        for y in 0..ns.len() {
            for x in 0..ns[0].len() {
                if ns[y][x] {
                    size += 1;
                }
            }
        }
        shape_sizes.push(size);
    }

    for r in regions.lines() {
        let mut itr = r.split(":");
        let size = itr.next().unwrap();
        let (width, height) = size.split_once("x").unwrap();
        let width: i64 = width.parse().unwrap();
        let height: i64 = height.parse().unwrap();

        let shape_nums: Vec<i64> = itr
            .next()
            .map(|nums| {
                nums.trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .unwrap();

        let area = width * height;
        let mut required_area = 0;
        for (idx, &num) in shape_nums.iter().enumerate() {
            required_area += shape_sizes[idx] * num;
        }

        if required_area > area {
            continue;
        }

        let area = vec![vec![false; width as usize]; height as usize];

        if top_dfs(&fr_shapes, &shape_nums, area) {
            res += 1;
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
}

#[test]
fn test_part1() {
    let file = file!();
    let nr = (&Path::new(file).file_stem().unwrap().to_str().unwrap()[3..])
        .parse::<i32>()
        .unwrap_or(0);
    assert_eq!(2, part1("test".to_string() + &nr.to_string() + ".txt"));
}
