use itertools::*;
use proconio::{fastout, input};

fn f(senbei: &[Vec<usize>]) -> usize {
    let r = senbei.len();
    let c = senbei[0].len();
    (0..c)
        .map(|j| (0..r).map(|i| senbei[i][j]).sum::<usize>())
        .map(|x| x.max(r - x))
        .sum::<usize>()
}

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        s: [[usize; c]; r],
    }

    let ans = (0..1 << r)
        .map(|bit| {
            let flipped = (0..r)
                .map(|i| {
                    if bit & (1 << i) != 0 {
                        (0..c).map(|j| s[i][j] ^ 1).collect_vec()
                    } else {
                        s[i].clone()
                    }
                })
                .collect_vec();
            f(&flipped)
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
