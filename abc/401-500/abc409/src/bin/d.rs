use itertools::*;
use proconio::{fastout, input, marker::Chars};

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
        s: Chars,
    }
    let l_opt = s.iter().tuple_windows().position(|(x, y)| x > y);

    if let Some(l) = l_opt {
        let r = (l + 1..n).find(|&i| s[l] < s[i]).unwrap_or(n);
        let ans = (0..l)
            .map(|i| s[i])
            .chain((l + 1..r).map(|i| s[i]))
            .chain((l..l + 1).map(|i| s[i]))
            .chain((r..n).map(|i| s[i]))
            .collect::<String>();

        println!("{}", ans);
    } else {
        let ans = s.iter().collect::<String>();
        println!("{}", ans);
    }
}
