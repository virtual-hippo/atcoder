use itertools::*;
use proconio::{fastout, input};

// r 以下かつ桁数が同じヘビ数の個数を求める
fn f(r: usize) -> usize {
    let s = r.to_string().chars().map(|ch| (ch as u8 - b'0') as usize).collect_vec();
    if s.len() == 1 {
        return 0;
    }

    let mut res = 0;
    for i in 1..s.len() {
        if s[i] >= s[0] {
            res += s[0].pow((s.len() - i) as u32);
            break;
        }

        if i == s.len() - 1 {
            res += s[i] + 1;
        } else {
            res += s[i] * s[0].pow((s.len() - i - 1) as u32);
        }
    }

    for x in 1..s[0] {
        res += x.pow((s.len() - 1) as u32);
    }

    res + f(10_usize.pow(s.len() as u32 - 1) - 1)
}

fn is_snake_number(x: usize) -> bool {
    let s = x.to_string().chars().map(|ch| ch as u8 - b'0').collect_vec();
    (1..s.len()).all(|i| s[i] < s[0])
}

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let ans = f(r) - f(l) + if is_snake_number(l) { 1 } else { 0 };
    println!("{}", ans);
}
