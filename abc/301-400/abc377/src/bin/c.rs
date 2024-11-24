use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut set = HashSet::new();

    let max = n * n;

    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }
        let a = a - 1;
        let b = b - 1;
        set.insert((a, b));
        for v in [-2, -1, 1, 2_i64] {
            for u in [-2, -1, 1, 2_i64] {
                if v.abs() + u.abs() == 3 {
                    let ii = a + v;
                    let jj = b + u;
                    if 0 <= ii && ii < n as i64 && 0 <= jj && jj < n as i64 {
                        set.insert((ii, jj));
                    }
                }
            }
        }
    }
    let ans = max - set.len();
    println!("{}", ans);
}
