use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut map = HashMap::new();
    for (i, v) in s.chars().enumerate() {
        map.insert(v, i);
    }
    let mut prev = 0;
    let mut ans = 0;
    for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars() {
        if let Some(v) = map.get(&ch) {
            if ch == 'A' {
            } else {
                if *v > prev {
                    ans += *v - prev;
                } else {
                    ans += prev - *v;
                }
            }
            prev = *v;
        }
    }
    println!("{}", ans);
}
