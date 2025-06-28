use proconio::{fastout, input, marker::Chars};

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

    let mut s = s;
    for i in 0..s.len() - 1 {
        (s[i], s[i + 1]) = (s[i + 1], s[i]);
        if s == t {
            println!("Yes");
            return;
        }
        (s[i], s[i + 1]) = (s[i + 1], s[i]);
    }

    println!("No");
}
