use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        for j in 1..k + 1 {
            ans += i * 100 + j;
        }
    }
    println!("{}", ans);
}
