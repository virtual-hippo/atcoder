use proconio::{fastout, input};
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Square {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Square {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
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

    let mut koho = vec![];
    {
        push_one_square(
            &mut rng,
            (0, 50_000),
            (0, 50_000),
            (10_000, 50_000),
            (10_000, 50_000),
            &saba,
            &iwashi,
            &mut koho,
        );
    }

    {
        let (square0, score0) = push_one_square(
            &mut rng,
            (0, 25_000),
            (0, 25_000),
            (10_000, 25_000),
            (10_000, 25_000),
            &saba,
            &iwashi,
            &mut koho,
        );
        let (square1, score1) = push_one_square(
            &mut rng,
            (50_001, 75_000),
            (50_001, 75_000),
            (10_000, 25_000),
            (10_000, 25_000),
            &saba,
            &iwashi,
            &mut koho,
        );
        koho.push((
            vec![
                (square0.x, square0.y),
                (square0.x + square0.w, square0.y),
                (square0.x + square0.w, square0.y + square0.h - 1),
                (square1.x + 1, square0.y + square0.h - 1),
                (square1.x + 1, square1.y),
                (square1.x + square1.w, square1.y),
                (square1.x + square1.w, square1.y + square1.h),
                (square1.x, square1.y + square1.h),
                (square1.x, square0.y + square0.h),
                (square0.x, square0.y + square0.h),
            ],
            score0 + score1,
        ));
    }

    let ans = {
        let mut tmp = koho[0].clone();
        for i in 1..koho.len() {
            let (ami, score) = &koho[i];
            if check_ami(&ami) && tmp.1 < *score {
                tmp = (ami.clone(), *score);
            }
        }
        tmp.0
    };

    println!("{}", ans.len());
    for (x, y) in ans.iter() {
        println!("{} {}", x, y);
    }
}

fn check_ami(ami: &[(usize, usize)]) -> bool {
    let length = {
        let mut length = 0;
        for i in 1..ami.len() {
            let x_diff = (ami[i].0 as i64 - ami[i - 1].0 as i64).abs();
            let y_diff = (ami[i].1 as i64 - ami[i - 1].1 as i64).abs();
            length += x_diff + y_diff;
        }
        let x_diff = (ami[0].0 as i64 - ami[ami.len() - 1].0 as i64).abs();
        let y_diff = (ami[0].1 as i64 - ami[ami.len() - 1].1 as i64).abs();
        length += x_diff + y_diff;

        length
    };
    if length >= 4 * 100_000 {
        return false;
    }
    let mut set = HashSet::new();
    for p in ami.iter() {
        set.insert(*p);
    }
    if set.len() != ami.len() {
        return false;
    }
    true
}

fn push_one_square(
    rng: &mut rand::prelude::ThreadRng,
    x_range: (usize, usize),
    y_range: (usize, usize),
    w_range: (usize, usize),
    h_range: (usize, usize),
    saba: &Vec<(usize, usize)>,
    iwashi: &Vec<(usize, usize)>,
    koho: &mut Vec<(Vec<(usize, usize)>, i64)>,
) -> (Square, i64) {
    let (square, score) = solve0(rng, x_range, y_range, w_range, h_range, &saba, &iwashi);

    koho.push((
        vec![
            (square.x, square.y),
            (square.x + square.w, square.y),
            (square.x + square.w, square.y + square.h),
            (square.x, square.y + square.h),
        ],
        score,
    ));

    (square, score)
}

fn solve0(
    rng: &mut rand::prelude::ThreadRng,
    x_range: (usize, usize),
    y_range: (usize, usize),
    w_range: (usize, usize),
    h_range: (usize, usize),
    saba: &Vec<(usize, usize)>,
    iwashi: &Vec<(usize, usize)>,
) -> (Square, i64) {
    let mut square = Square::new(0, 0, 0, 0);
    let mut max_score = 0;
    for _ in 0..5000 {
        let x: usize = rng.gen_range(x_range.0..x_range.1);
        let y: usize = rng.gen_range(y_range.0..y_range.1);
        let w: usize = rng.gen_range(w_range.0..w_range.1);
        let h: usize = rng.gen_range(h_range.0..h_range.1);

        let a = count_fish_in_grid(x, y, w, h, saba);
        let b = count_fish_in_grid(x, y, w, h, iwashi);
        let now = 0.max(a - b + 1);

        if now > max_score {
            square = Square::new(x, y, w, h);
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
fn count_fish_in_grid(x: usize, y: usize, w: usize, h: usize, fish: &Vec<(usize, usize)>) -> i64 {
    let mut ret = 0;
    for (i, j) in fish.iter() {
        if x <= *i && *i < x + w {
            if y <= *j && *j < y + h {
                ret += 1;
            }
        }
    }
    ret
}
