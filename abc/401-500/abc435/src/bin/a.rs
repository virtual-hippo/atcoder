use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ans = ((1 + n) * n) / 2;
    println!("{}", ans);
}
