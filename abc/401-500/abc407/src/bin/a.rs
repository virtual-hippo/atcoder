use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let v = a as f64 / b as f64;
    let c = (a / b) as f64;
    let d = (a / b) as f64 + 1.0;
    let ans = if v - c < d - v { a / b } else { a / b + 1 };

    println!("{}", ans);
}
