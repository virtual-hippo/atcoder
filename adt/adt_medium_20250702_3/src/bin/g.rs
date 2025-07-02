use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _n: usize,
        r: i64,
        c: i64,
        s: String,
    }

    let mut takahashi = (r, c);
    let mut visited = HashSet::new();
    let mut now = (0, 0);

    for c in s.chars() {
        let d = match c {
            'N' => (1, 0),
            'W' => (0, 1),
            'S' => (-1, 0),
            'E' => (0, -1),
            _ => unreachable!(),
        };
        takahashi.0 += d.0;
        takahashi.1 += d.1;

        visited.insert(now);
        now.0 += d.0;
        now.1 += d.1;

        if visited.contains(&takahashi) {
            print!("1");
        } else {
            print!("0");
        }
    }

    println!("");
}
