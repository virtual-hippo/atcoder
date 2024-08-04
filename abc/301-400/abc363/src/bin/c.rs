use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

fn is_kaibun(s: &[&char]) -> bool {
    let s_len = s.len();
    for i in 0..s_len / 2 {
        if s[i] != s[s_len - 1 - i] {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut ans = 0;
    let mut set = HashSet::new();

    for vec in s.iter().permutations(n) {
        set.insert(vec);
    }

    for vec in set.iter() {
        let mut flag = true;
        for i in 0..(n - k + 1) {
            let ret = is_kaibun(&vec[i..i + k]);
            if ret {
                flag = false;
                break;
            }
        }
        if flag {
            ans += 1;
        }
    }
    println!("{}", ans);
}
