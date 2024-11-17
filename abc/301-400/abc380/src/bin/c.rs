use std::usize;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }
    let mut is_one = false;
    let mut lr = vec![];

    let mut l = 0;
    for (i, c) in s.iter().enumerate() {
        if !is_one && *c == '1' {
            is_one = true;
            l = i;
        }
        if is_one && *c == '0' {
            is_one = false;
            lr.push((l, i));
        }
    }
    if is_one {
        lr.push((l, s.len()));
    }

    let len = lr[k - 1].1 - lr[k - 1].0;
    let (ll, rr) = (lr[k - 2].1, lr[k - 2].1 + len);
    for (i, c) in s.iter().enumerate() {
        if ll <= i && i < rr {
            print!("1");
            continue;
        }
        if lr[k - 1].0 <= i && i < lr[k - 1].1 {
            print!("0");
            continue;
        }
        print!("{}", c);
    }
}
