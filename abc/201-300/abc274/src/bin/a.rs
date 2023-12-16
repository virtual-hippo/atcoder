use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let ans = b / a;
    println!("{:.3}", ans);
}
