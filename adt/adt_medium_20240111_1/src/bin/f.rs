use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let mut current = (0_i64, 0_i64);
    let mut set = HashSet::new();
    for ch in s.chars() {
        if set.contains(&current) {
            println!("Yes");
            return;
        }
        set.insert(current);
        match ch {
            'R' => current.0 += 1,
            'L' => current.0 -= 1,
            'U' => current.1 += 1,
            'D' => current.1 -= 1,
            _ => unreachable!(),
        }
    }
    if set.contains(&current) {
        println!("Yes");
    } else {
        println!("No");
    }
}
