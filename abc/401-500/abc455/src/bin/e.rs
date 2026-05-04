use proconio::input;
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let f = |co: &[i64]| -> i64 {
        s.chars()
            .map(|ch| (ch as u8 - b'A') as usize)
            .scan((0, HashMap::new()), |(x, d), i| {
                let k = *x;
                *d.entry(k).or_insert(0) += 1;

                *x += co[i];
                let val = *d.get(&x).unwrap_or(&0);
                Some(val)
            })
            .sum()
    };

    let ans = f(&[0, 0, 0]) - f(&[1, -1, 0]) - f(&[1, 0, -1]) - f(&[0, 1, -1])
        + f(&[2_000_001, -2_000_000, -1]) * 2;
    println!("{}", ans);
}
