use itertools::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let is = {
        let mut ret = vec![vec![n + 1; n + 2]; 26];
        for i in 0..n {
            let ch_i = (s[i] as u8 - b'a') as usize;
            ret[ch_i][i] = i + 1;
        }

        iproduct!(0..26, (0..n + 1).rev()).for_each(|(ch_i, i)| {
            ret[ch_i][i] = ret[ch_i][i].min(ret[ch_i][i + 1]);
        });

        ret
    };

    let ans = (0..n).fold(0, |acc, l| {
        let r = t.iter().fold(l, |r, &ch| {
            let ch_i = (ch as u8 - b'a') as usize;
            is[ch_i][r]
        });
        acc + r - l - 1
    });
    println!("{}", ans);
}
