use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: f64,
    }
    let ans = d / 100.0;
    println!("{}", ans);
}
