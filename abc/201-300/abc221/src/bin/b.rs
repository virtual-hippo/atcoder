use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut is_swapped = (false, false);
    for i in 0..s.len() {
        if s[i] == t[i] {
            continue;
        }
        if is_swapped.0 && is_swapped.1 {
            println!("No");
            return;
        }
        if i < s.len() - 1 && s[i] == t[i + 1] && s[i + 1] == t[i] {
            is_swapped.0 = true;
            continue;
        } else if 0 < i && s[i] == t[i - 1] && s[i - 1] == t[i] {
            is_swapped.1 = true;
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
