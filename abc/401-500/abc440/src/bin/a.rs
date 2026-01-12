use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let ans = x * 1 << y;
    println!("{}", ans);
}
