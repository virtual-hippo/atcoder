use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut mode = false;
    for i in 0..n {
        if mode == false && s[i] == ',' {
            print!(".");
        } else {
            print!("{}", s[i]);
        }
        if s[i] == '"' {
            mode = !mode;
        }
    }
}

