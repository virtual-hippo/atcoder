use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut current = (0, 0);
    let mut history = HashSet::with_capacity(n);
    history.insert(current);
    for i in 0..n {
        match s[i] {
            'R' => current.0 += 1,
            'L' => current.0 -= 1,
            'U' => current.1 += 1,
            'D' => current.1 -= 1,
            _ => unreachable!(),
        }
        history.insert(current);
    }
    if history.len() == n + 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
