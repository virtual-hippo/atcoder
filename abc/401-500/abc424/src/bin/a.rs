use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a == b {
        println!("Yes");
        return;
    }

    if c == b {
        println!("Yes");
        return;
    }

    if a == c {
        println!("Yes");
        return;
    }

    println!("No");
}
