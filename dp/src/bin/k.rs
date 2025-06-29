use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; k + 10];

    for i in 1..=k {
        for j in 0..n {
            if i >= a[j] {
                dp[i] |= !dp[i - a[j]];
            }
        }
    }

    if dp[k] {
        println!("First");
    } else {
        println!("Second");
    }
}
