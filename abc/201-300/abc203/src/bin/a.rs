use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    if a == b {
        println!("{}", c);
    } else if c == b {
        println!("{}", a);
    } else if a == c {
        println!("{}", b);
    } else {
        println!("{}", 0);
    }
}
