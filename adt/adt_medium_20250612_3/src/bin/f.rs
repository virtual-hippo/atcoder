use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    // dp[i][j]:=数列の先頭から i 項まで決めた際に、総和が j であるような数列の決め方の総数

    let mut dp = [[0; 2510]; 55];
    for j in 1..=m {
        dp[1][j] = 1;
    }

    for i in 2..=n {
        for j in 1..=k {
            for kk in 1..=m {
                if j >= kk {
                    dp[i][j] += dp[i - 1][j - kk];
                    dp[i][j] %= 998244353;
                }
            }
        }
    }

    let ans = (n..=k).map(|i| dp[n][i]).sum::<usize>() % 998244353;
    println!("{}", ans);
}
