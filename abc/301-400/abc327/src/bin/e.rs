use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }
    let c = 0.9;
    let mut dp = vec![0.0; n + 1];

    for i in 0..n {
        dp[i + 1] = c * dp[i] + p[i];
        for j in (0..i).rev() {
            dp[j + 1] = dp[j + 1].max(c * dp[j] + p[i]);
        }
    }
    let mut ans = -1200.0;
    let mut w = 0.0;
    for i in 1..n + 1 {
        w = c * w + 1.0;
        let current = dp[i] / w - 1200.0 / (i as f64).sqrt();
        if ans < current {
            ans = current;
        }
    }
    println!("{}", ans);
}
