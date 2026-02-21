use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        n: usize,
        pieces: [(usize,usize); n],
    }

    let ord_h = (0..n).sorted_by_key(|&i| std::cmp::Reverse(pieces[i].0)).collect_vec();
    let ord_w = (0..n).sorted_by_key(|&i| std::cmp::Reverse(pieces[i].1)).collect_vec();

    let mut cursor = (0, 0);
    let mut ans = vec![(0, 0); n];
    let mut used = vec![false; n];
    let mut hi = 0;
    let mut wi = 0;

    for _ in 0..n {
        while used[ord_h[hi]] {
            hi += 1;
        }
        while used[ord_w[wi]] {
            wi += 1;
        }

        let i = {
            let phi = ord_h[hi];
            let pwi = ord_w[wi];

            if pieces[phi].0 + cursor.0 == h {
                phi
            } else {
                pwi
            }
        };
        used[i] = true;
        ans[i] = cursor;

        let (hh, ww) = pieces[i];

        if hh + cursor.0 == h {
            cursor.1 += ww;
        } else {
            cursor.0 += hh;
        }
    }

    for i in 0..n {
        println!("{} {}", ans[i].0 + 1, ans[i].1 + 1);
    }
}
