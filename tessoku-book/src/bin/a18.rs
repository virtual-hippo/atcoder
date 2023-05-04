use proconio::input;

fn main() {
    input! {
        (n, s): (usize, usize),
        a: [usize; n],
    }
    let mut dp = vec![vec![false; 10_000 * (n+1)]; n+1];
    for i in 0..n+1 {
        dp[i][0] = true;
    }
    for i in 1..n+1 {
        for j in 0..s+1 {
            if dp[i-1][j] {
                dp[i][j] = true;
                dp[i][j+a[i-1]] = true;
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}

