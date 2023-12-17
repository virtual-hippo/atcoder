use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }
    s.sort();
    t.sort();
    let near = |a: char, b: char| -> bool { b as u8 - a as u8 == 1 || b as u8 - a as u8 == 4 };
    if near(s[0], s[1]) == near(t[0], t[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
