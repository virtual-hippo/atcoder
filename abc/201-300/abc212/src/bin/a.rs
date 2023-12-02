use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let ans = if 0 < a && b == 0 {
        "Gold"
    } else if a == 0 && 0 < b {
        "Silver"
    } else {
        "Alloy"
    };
    println!("{}", ans);
}
