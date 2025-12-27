use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let s = s.chars().map(|c| c as u8 - b'0').collect_vec();
    let t = t.chars().map(|c| c as u8 - b'0').collect_vec();

    let mut ans = usize::MAX;

    for i in 0..n {
        if i + m - 1 >= n {
            break;
        }

        let mut now = 0;
        for j in 0..m {
            let k = j + i;

            now += ((10 + s[k] - t[j]) % 10) as usize;
        }
        eprintln!("{}", now);

        ans = ans.min(now);
    }

    println!("{}", ans);
}
