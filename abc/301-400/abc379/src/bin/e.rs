use itertools::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let a = s.iter().map(|&c| c as u64 - b'0' as u64).collect::<Vec<_>>();
    let b = a.iter().enumerate().map(|(i, &x)| (i + 1) as u64 * x).collect_vec();
    let s = std::iter::once(0_u64)
        .chain(b.iter().scan(0_u64, |sum, &x| {
            *sum = *sum + x;
            Some(*sum)
        }))
        .collect::<Vec<_>>();

    let mut ch = vec![];
    let mut now = 0_u64;
    for i in 0..n {
        let v = s[n - i] - s[0] + now;
        now = v / 10;
        let m = v % 10;
        ch.push((m as u8 + b'0') as char);
    }

    let rev_ch = ch.into_iter().rev().collect::<String>();
    let ans = if now == 0 { rev_ch } else { format!("{}{}", now, rev_ch) };
    println!("{}", ans);
}
