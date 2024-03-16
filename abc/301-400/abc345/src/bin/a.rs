use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    if s[0] == '<' && s[s.len() - 1] == '>' {
        for i in 1..(s.len() - 1) {
            if s[i] != '=' {
                println!("No");
                return;
            }
        }
    } else {
        println!("No");
        return;
    }
    println!("Yes");
}
