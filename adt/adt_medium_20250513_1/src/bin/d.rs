use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    if s == t {
        println!("{}", 0);
        return;
    }

    for i in 0..s.len().max(t.len()) {
        if i < s.len() && i < t.len() {
            if s[i] == t[i] {
                continue;
            } else {
                println!("{}", i + 1);
                return;
            }
        }
        if i < s.len() {
            println!("{}", i + 1);
            return;
        }
        if i < t.len() {
            println!("{}", i + 1);
            return;
        }
    }
}
