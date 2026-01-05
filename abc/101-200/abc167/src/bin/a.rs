use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ans = (0..s.len()).all(|i| s[i] == t[i]);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
