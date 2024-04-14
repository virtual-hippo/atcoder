use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
    }
    let v: i64 = a.iter().sum();
    let ans = 0 - v;
    println!("{}", ans);
}
