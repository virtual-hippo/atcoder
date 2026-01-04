use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = 100.0 - (b * 100) as f64 / a as f64;
    println!("{}", ans);
}
