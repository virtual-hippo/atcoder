use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: Chars,
    }

    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        println!("Weak");
        return;
    }

    let mut cnt = 0;
    for i in 1..4 {
        if x[i] as u8 == x[i - 1] as u8 + 1 {
            cnt += 1
        } else if x[i] == '0' && x[i - 1] == '9' {
            cnt += 1;
        }
    }
    if cnt == 3 {
        println!("Weak");
        return;
    }
    println!("Strong");
}
