use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut counter = HashMap::new();
    for i in 0..s.len() {
        *counter.entry(s[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    let mut is = false;

    for i in 0..s.len() {
        if let Some(v) = counter.get_mut(&s[i]) {
            *v -= 1;
            if *v > 0 {
                is = true;
            }
            ans += s.len() - (i + 1) - *v;
        }
    }
    if is {
        ans += 1;
    }
    println!("{}", ans);
}
