use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        sx: i64,
        sy: i64,
    }
    let mut sx = sx;
    let mut sy = sy;

    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        x_map.entry(x).or_insert_with(BTreeSet::new).insert(y);
        y_map.entry(y).or_insert_with(BTreeSet::new).insert(x);
    }

    let mut house: HashSet<(i64, i64)> = HashSet::new();

    let mut remove_coordinates = |key: i64,
                                  l: i64,
                                  r: i64,
                                  map: &mut HashMap<i64, BTreeSet<i64>>,
                                  other_map: &mut HashMap<i64, BTreeSet<i64>>,
                                  is_x: bool| {
        if let Some(set) = map.get_mut(&key) {
            let range = set.range(l..=r).map(|&val| val).collect_vec();
            for coord in range {
                if is_x {
                    house.insert((key, coord));
                } else {
                    house.insert((coord, key));
                }
                set.remove(&coord);
                if let Some(other_set) = other_map.get_mut(&coord) {
                    other_set.remove(&key);
                }
            }
        }
    };

    for _ in 0..m {
        input! {
            d: char,
            c: i64,
        }

        let (nx, ny) = match d {
            'U' => (sx, sy + c),
            'D' => (sx, sy - c),
            'L' => (sx - c, sy),
            'R' => (sx + c, sy),
            _ => unreachable!(),
        };

        match d {
            'U' | 'D' => {
                remove_coordinates(sx, sy.min(ny), sy.max(ny), &mut x_map, &mut y_map, true)
            }
            'L' | 'R' => {
                remove_coordinates(sy, sx.min(nx), sx.max(nx), &mut y_map, &mut x_map, false)
            }
            _ => unreachable!(),
        }

        sx = nx;
        sy = ny;
    }

    println!("{} {} {}", sx, sy, house.len());
}
