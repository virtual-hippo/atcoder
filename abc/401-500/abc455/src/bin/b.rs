use itertools::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let ans = iproduct!(0..h, 0..w)
        .map(|(h1, w1)| {
            iproduct!(h1..h, w1..w)
                .filter(|&(h2, w2)| iproduct!(h1..=h2, w1..=w2).all(|(i, j)| s[i][j] == s[h1 + h2 - i][w1 + w2 - j]))
                .count()
        })
        .sum::<usize>();

    println!("{}", ans);
}
