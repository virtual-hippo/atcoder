use proconio::{fastout, input};
use superslice::Ext;

pub fn binary_search<F: Fn(i64) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
        q: usize,
        mut a: [i64; n],
    }
    a.sort();

    for _ in 0..q {
        input! {
            b: i64,
            k: usize,
        }
        let f = |val: i64| {
            let r = a.lower_bound(&(b + val));
            let l = a.upper_bound(&(b - val));
            let kk = r - l + 1;
            kk <= k
        };
        let ret = binary_search((0, 300_000_000), f);
        println!("{}", ret.0);
    }
}
