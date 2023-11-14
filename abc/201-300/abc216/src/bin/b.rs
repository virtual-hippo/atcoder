use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            s: String,
            t: String,
        }
        let pair = (s, t);
        if set.contains(&pair) {
            println!("Yes");
            return;
        } else {
            set.insert(pair);
        }
    }
    println!("No");
}
