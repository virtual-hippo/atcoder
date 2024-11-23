use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    if n % 2 == 1 {
        println!("No");
        return;
    }

    let mut set = HashSet::new();

    for i in 0..(n / 2) {
        let i = 2 * i;
        if set.contains(&s[i]) {
            println!("No");
            return;
        }
        if s[i] != s[i + 1] {
            println!("No");
            return;
        }
        set.insert(s[i]);
    }
    println!("Yes");
}
