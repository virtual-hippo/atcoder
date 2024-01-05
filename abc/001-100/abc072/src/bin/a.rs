use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        t: i64,
    }
    let ans = (x - t).max(0);
    println!("{}", ans);
}
