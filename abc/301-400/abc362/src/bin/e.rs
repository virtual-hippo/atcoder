use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = [[[ModInt998244353::new(0); 81]; 81]; 81];

    for i in 0..n {
        for j in 0..i {
            dp[i][j][2] += ModInt998244353::new(1);
        }
        for j in 0..i {
            for k in 2..n {
                if dp[i][j][k] == ModInt998244353::new(0) {
                    continue;
                }
                for l in ((i + 1)..n).filter(|&l| a[l] - a[i] == a[i] - a[j]) {
                    dp[l][i][k + 1] += dp[i][j][k];
                }
            }
        }
    }

    let mut ans = vec![ModInt998244353::new(0); n + 1];
    ans[1] = ModInt998244353::new(n);

    for k in 2..n + 1 {
        for i in 0..n {
            for j in 0..i {
                ans[k] += dp[i][j][k];
            }
        }
    }

    for i in 1..n + 1 {
        print!("{} ", ans[i]);
    }
}
