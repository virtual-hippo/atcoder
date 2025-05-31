use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
    }

    let cnt_1 = s.iter().filter(|&&c| c == '1').count() as i64;
    let mut c = vec![0_i64; n + 1];

    for i in 0..n {
        c[i + 1] = if s[i] == '0' { 1 } else { -1 } + c[i];
    }

    let mut res = 0;
    let mut mx = 0;

    for i in 0..n + 1 {
        res = res.min(c[i] - mx);
        mx = mx.max(c[i]);
    }
    let ans = res + cnt_1;
    println!("{}", ans);
}

fn solve2() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![[usize::MAX; 3]; n + 1];
    // 0: 1 区間に入る前
    // 1: 1 区間の中
    // 2: 1 区間を出た
    dp[0][0] = 0;
    dp[0][1] = 0;
    dp[0][2] = 0;

    for i in 1..n + 1 {
        let ns = s[i - 1];
        dp[i][0] = dp[i - 1][0] + if ns == '0' { 0 } else { 1 };

        dp[i][1] = dp[i][1].min(dp[i - 1][0] + if ns == '0' { 1 } else { 0 });
        dp[i][1] = dp[i][1].min(dp[i - 1][1] + if ns == '0' { 1 } else { 0 });

        dp[i][2] = dp[i][2].min(dp[i - 1][1] + if ns == '0' { 0 } else { 1 });
        dp[i][2] = dp[i][2].min(dp[i - 1][2] + if ns == '0' { 0 } else { 1 });
    }
    println!("{}", dp[n].iter().min().unwrap());
}
