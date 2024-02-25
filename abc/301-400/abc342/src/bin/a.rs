use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    for (i, &ch) in s.iter().enumerate() {
        if i == 0 {
            if ch != s[i + 1] && ch != s[i + 2] {
                println!("{}", 1);
                return;
            }
            continue;
        }
        if i == s.len() - 1 {
            println!("{}", i + 1);
            return;
        }
        if ch != s[i - 1] && ch != s[i + 1] {
            println!("{}", i + 1);
            return;
        }
    }
}
