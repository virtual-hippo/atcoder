use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
    }
    let ans = 100 - (r % 100);
    println!("{}", ans);
}
