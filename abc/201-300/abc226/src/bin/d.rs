use itertools::Itertools;
use num_integer::gcd;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        points.push((x, y));
    }
    let mut set = HashSet::with_capacity(n * n);
    for (p0, p1) in points.iter().tuple_combinations() {
        let (dx, dy) = (p0.0 - p1.0, p0.1 - p1.1);
        let g = gcd(dx.max(-dx), dy.max(-dy));
        set.insert((dx / g, dy / g));
        set.insert((-dx / g, -dy / g));
    }

    println!("{}", set.len());
}
