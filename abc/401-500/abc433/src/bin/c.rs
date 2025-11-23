use itertools::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_u64;

    let run_length = s.iter().copied().dedup_with_count().collect_vec();

    for (i, (cnt, ch)) in run_length.iter().copied().enumerate() {
        if i == run_length.len() - 1 {
            break;
        }

        if ch as u8 + 1 == run_length[i + 1].1 as u8 {
            let mn = cnt.min(run_length[i + 1].0);
            ans += mn as u64;
        }
    }

    println!("{}", ans);
}
