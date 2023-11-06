use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    for i in 1..n {
        if s[i] == 'a' && s[i - 1] == 'b' {
            println!("Yes");
            return;
        } else if s[i] == 'b' && s[i - 1] == 'a' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
