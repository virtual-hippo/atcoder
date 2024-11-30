#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    if n < 3 {
        println!("0");
        return;
    }

    let mut ruisekiwa = vec![vec![0; n + 1]; 26];
    for i in 0..n {
        let k = (s[i] as u8 - b'A') as usize;
        ruisekiwa[k][i + 1] += 1;
        for j in 0..26 {
            ruisekiwa[j][i + 1] += ruisekiwa[j][i];
        }
    }

    let mut ans = 0_u64;
    for i in 1..n - 1 {
        for j in 0..26 {
            ans += (ruisekiwa[j][i] - ruisekiwa[j][0]) * (ruisekiwa[j][n] - ruisekiwa[j][i + 1]);
        }
    }
    println!("{}", ans);
}
