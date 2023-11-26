use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut r_map = HashMap::new();
    let mut c_map = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                *r_map.entry(i).or_insert(0) += 1;
                *c_map.entry(j).or_insert(0) += 1;
            }
        }
    }

    let mut ans = 0_u64;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                if let Some(&cnt) = r_map.get(&i) {
                    if cnt > 1 {
                        if let Some(&cnt2) = c_map.get(&j) {
                            if cnt2 > 1 {
                                ans += (cnt - 1) * (cnt2 - 1);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
