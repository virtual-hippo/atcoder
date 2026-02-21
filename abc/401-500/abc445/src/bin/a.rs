use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    if s[0] == s[s.len() - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
