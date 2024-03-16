use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    if x >= 0 {
        if x % 10 == 0 {
            println!("{}", x / 10);
        } else {
            println!("{}", x / 10 + 1);
        }
    } else {
        if x % 10 == 0 {
            println!("{}", x / 10);
        } else {
            println!("{}", x / 10);
        }
    }
}
