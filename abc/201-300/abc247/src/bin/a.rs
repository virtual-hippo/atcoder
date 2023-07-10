use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    print!("0");
    (0..3).for_each(|i| print!("{}", s[i]));
}

