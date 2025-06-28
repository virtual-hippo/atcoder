use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: [usize; n],
    }

    if s[0] * 2 >= s[n - 1] {
        println!("{}", 2);
        return;
    }

    let t = s
        .iter()
        .map(|v| *v)
        .sorted()
        .filter(|&v| s[0] < v && v < s[n - 1])
        .unique()
        .collect_vec();

    let mut right = s[0];
    let mut ans = 1;

    for i in 0..t.len() {
        if i == 0 && t[i] > 2 * right {
            println!("{}", -1);
            return;
        }
        if i == 0 {
            continue;
        }

        if i > 0 && t[i] > 2 * right {
            if t[i - 1] <= 2 * right {
                right = t[i - 1];
            }
            ans += 1;
        }

        if s[n - 1] <= 2 * right {
            println!("{}", ans + 1);
            return;
        }
    }

    if t.len() > 0 && t[t.len() - 1] <= 2 * right {
        right = t[t.len() - 1];
        ans += 1;
    }
    if s[n - 1] <= 2 * right {
        println!("{}", ans + 1);
        return;
    }

    println!("{}", -1);
}
