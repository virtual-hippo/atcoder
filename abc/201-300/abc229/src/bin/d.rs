use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let n = s.len();

    let cnt = (0..n).fold(vec![0_usize], |mut acc, i| {
        let x = acc[acc.len() - 1] + if s[i] == '.' { 1 } else { 0 };
        acc.push(x);
        acc
    });

    let ans: usize = (0..n)
        .scan(0_usize, |r, l| {
            while *r < n && cnt[*r + 1] - cnt[l] <= k {
                *r += 1;
            }
            Some(*r - l)
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
