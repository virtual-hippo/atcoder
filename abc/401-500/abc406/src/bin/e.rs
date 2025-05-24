// 桁DP (https://algo-logic.info/digit-dp/)
// 解説AC

use ac_library::*;
use proconio::{fastout, input};

type MInt = ModInt998244353;
const M: usize = 60;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        solve(n, k);
    }
}

fn solve(n: usize, k: usize) {
    // dp[i][j][s][p] = 上から i 桁決めて popcount が s 未満が確定したかどうか, p bit 目 を立てたか
    let mut dp = [[[[MInt::new(0); M + 1]; 2]; 2]; M + 1];
    dp[M][0][0][0] = MInt::new(1);

    let n = n + 1;

    for i in (0..M).rev() {
        for j in 0..2 {
            for s in 0..2 {
                for p in 0..k + 1 {
                    let now = dp[i + 1][j][s][p];
                    if now == MInt::new(0) {
                        continue;
                    }

                    for a in 0..2 {
                        let mut ns = s;
                        let np = p + a;

                        if s == 0 {
                            // n の i 桁目より小さい場合
                            if a < (n >> i) & 1 {
                                ns = 1;
                            }
                            if a > (n >> i) & 1 {
                                continue;
                            }
                        }

                        if np > k {
                            continue;
                        }
                        dp[i][j][ns][np] += now;
                        if j == 0 && a == 1 {
                            dp[i][1][ns][np] += now * (1_u64 << i);
                        }
                    }
                }
            }
        }
    }

    let ans = dp[0][1][1][k];
    println!("{}", ans.val());
}
