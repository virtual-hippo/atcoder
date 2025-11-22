use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if c >= a {
        if d >= b {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        println!("No");
    }
}
