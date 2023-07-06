use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    for i in 0..s.len() {
        let mut current = s.clone();
        if 0 < i {
            current[i] = s[i-1];
            current[i-1] = s[i];
        }
        let mut ret = true;
        for j in 0..s.len() {
            if current[j] != t[j] {
                ret = false;
            }
        }
        if ret {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

