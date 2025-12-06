use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        whb: [(usize,i64,i64); n],
    }

    let sw_half = whb.iter().map(|&(w, _, _)| w).sum::<usize>() / 2;

    let mut dp = vec![-1i64; sw_half + 1];
    dp[0] = 0;

    for i in 0..n {
        let (pw, ph, pb) = whb[i];

        let mut curr = vec![-1i64; sw_half + 1];

        for j in 0..=sw_half {
            if dp[j] != -1 {
                curr[j] = curr[j].max(dp[j] + pb);
            }

            if j >= pw && dp[j - pw] != -1 {
                curr[j] = curr[j].max(dp[j - pw] + ph);
            }
        }
        dp = curr;
    }

    let ans = dp.iter().max().unwrap();

    println!("{}", ans);
}
