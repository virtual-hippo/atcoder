use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i128,
    }
    let ans = if x >= 0 {
        x / 10
    } else {
        if x % 10 == 0 {
            x / 10
        } else {
            x / 10 - 1
        }
    };
    println!("{}", ans);
}
