use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for l in 0..n {
        for d in 1..=k {
            let i = l + d;
            if i > n {
                break;
            }
            let (a, b) = ab[i - 1];
            for pm in 0..=m.saturating_sub(b) {
                dp[i][pm + b] = dp[i][pm + b].max(dp[l][pm] + a);
            }
        }
    }

    let ans = dp.iter().flatten().max().unwrap();
    println!("{}", ans);
}
