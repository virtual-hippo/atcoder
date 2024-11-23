use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    if n == 1 && s[0] == '/' {
        println!("Yes");
        return;
    }
    if s.len() % 2 == 0 {
        println!("No");
        return;
    }

    if s[(n + 1) / 2 - 1] != '/' {
        println!("No");
        return;
    }
    for i in 0..((n + 1) / 2 - 1) {
        if s[i] != '1' {
            println!("No");
            return;
        }
    }

    for i in ((n + 1) / 2)..n {
        if s[i] != '2' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
