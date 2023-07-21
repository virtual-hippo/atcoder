use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    if s.len() != 8 {
        println!("No");
        return;
    }
    for i in 0..s.len() {
        let as_8 = s[i] as u8;
        if i == 0 || i == s.len() - 1 {
            if 65 <= as_8 && as_8 < 65 + 26 {
                //
            } else {
                println!("No");
                return;
            }
        } else if i == 1 {
            if 49 <= as_8 && as_8 < 48 + 10 {
                //
            } else {
                println!("No");
                return;
            }
        } else {
            if 48 <= as_8 && as_8 < 48 + 10 {
                //
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

