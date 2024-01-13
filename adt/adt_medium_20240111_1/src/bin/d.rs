use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    if s == t {
        println!("Yes");
        return;
    }
    for i in 0..s.len() - 1 {
        let mut current = s.clone();
        (current[i], current[i + 1]) = (s[i + 1], s[i]);
        if current == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
