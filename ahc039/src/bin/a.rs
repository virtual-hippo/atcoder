use std::collections::btree_map::Range;

use proconio::{fastout, input};
use rand::Rng;
struct Square {
    x: usize,
    y: usize,
    d: usize,
}

impl Square {
    fn new(x: usize, y: usize, d: usize) -> Self {
        Self { x, y, d }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let saba = fetch_fish(n);
    let iwashi = fetch_fish(n);

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let (square, _score) = solve0(
        &mut rng,
        (0, 50_000),
        (0, 50_000),
        (10_000, 50_000),
        &saba,
        &iwashi,
    );

    println!("4");
    println!("{} {}", square.x, square.y);
    println!("{} {}", square.x + square.d, square.y);
    println!("{} {}", square.x + square.d, square.y + square.d);
    println!("{} {}", square.x, square.y + square.d);
}

fn solve0(
    rng: &mut rand::prelude::ThreadRng,
    x_range: (usize, usize),
    y_range: (usize, usize),
    d_range: (usize, usize),
    saba: &Vec<(usize, usize)>,
    iwashi: &Vec<(usize, usize)>,
) -> (Square, i64) {
    let mut square = Square::new(0, 0, 0);
    let mut max_score = 0;
    for _ in 0..5000 {
        let x: usize = rng.gen_range(x_range.0..x_range.1);
        let y: usize = rng.gen_range(y_range.0..y_range.1);
        let d: usize = rng.gen_range(d_range.0..d_range.1);

        let a = count_fish_in_grid(x, y, d, saba);
        let b = count_fish_in_grid(x, y, d, iwashi);
        let now = 0.max(a - b + 1);

        if now > max_score {
            square = Square::new(x, y, d);
            max_score = now;
        }
    }
    (square, max_score)
}

fn fetch_fish(n: usize) -> Vec<(usize, usize)> {
    let mut fish = vec![];
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }
        fish.push((x, y));
    }
    fish
}

// O(5,000)
fn count_fish_in_grid(x: usize, y: usize, d: usize, fish: &Vec<(usize, usize)>) -> i64 {
    let mut ret = 0;
    for (i, j) in fish.iter() {
        if x <= *i && *i < x + d {
            if y <= *j && *j < y + d {
                ret += 1;
            }
        }
    }
    ret
}
