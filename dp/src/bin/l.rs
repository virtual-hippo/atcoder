use proconio::{fastout, input};

fn dfs(a: &[i64], l: usize, r: usize, dp: &mut Vec<Vec<i64>>) -> i64 {
    if dp[l][r] != 0 {
        return dp[l][r];
    }

    if l == r {
        return 0;
    }

    let res = i64::MIN.max(-dfs(a, l + 1, r, dp) + a[l]).max(-dfs(a, l, r - 1, dp) + a[r - 1]);
    dp[l][r] = res;
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![0; 3010]; 3010];

    let ans = dfs(&a, 0, n, &mut dp);
    println!("{}", ans);
}
