use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: f64,
    }
    let ans = (h * (12800000.0 + h)).sqrt();
    println!("{}", ans);
}
