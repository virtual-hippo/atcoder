use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut dp = vec![0; n + 1];

    for i in 0..=n {
        for &ai in a.iter() {
            if ai <= i {
                dp[i] = dp[i].max(i - dp[i - ai]);
            }
        }
    }

    let ans = dp[n];
    println!("{}", ans);
}
