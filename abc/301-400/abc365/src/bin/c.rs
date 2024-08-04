use proconio::{fastout, input};
use superslice::Ext;

pub fn binary_search<F: Fn(usize) -> bool>(
    initial_pos: (usize, usize),
    is_ok: F,
) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left, right)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = a[i] + s[i];
    }
    if s[n] <= m {
        println!("infinite");
        return;
    }
    let f = |val: usize| {
        let l = a.lower_bound(&val);
        s[l] + val * (n - l) <= m
    };
    let (ans, _) = binary_search((0, 1_000_000_000), f);

    println!("{}", ans);
}
