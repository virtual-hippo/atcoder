use itertools::*;
use proconio::{input, marker::*};
use std::usize;

fn main() {
    input! {
        n: usize,
        q: usize,
        cp: [(Usize1,Usize1); q],
    }

    let mut cards = (0..n).map(|_| (usize::MAX - 1, usize::MAX)).collect_vec();
    let mut piles = vec![1; n];

    for i in 0..q {
        let (c, p) = cp[i];

        if cards[c].0 == usize::MAX - 1 {
            piles[c] = 0;
        }
        let b = cards[c].0;
        if b < n {
            cards[b].1 = usize::MAX;
        }

        cards[c].0 = p;
        cards[p].1 = c;
    }

    let ans = (0..n)
        .map(|i| {
            if piles[i] == 0 {
                0
            } else {
                let mut cnt = 1;
                let mut p = i;
                while cards[p].1 != usize::MAX {
                    p = cards[p].1;
                    cnt += 1;
                }
                cnt
            }
        })
        .collect_vec();
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
