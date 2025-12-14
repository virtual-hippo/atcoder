use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let d = {
        let mut d = vec![vec![0; n]; n];
        for i in 0..n - 1 {
            for j in i + 1..n {
                input! {
                    dd: usize,
                }
                d[i][j] = dd;
                d[j][i] = dd;
            }
        }
        d
    };

    solve0(n, &d);
}

// bit DP による解法
pub fn solve0(n: usize, d: &Vec<Vec<usize>>) {
    let mut dp = vec![0; (1 << n) + 1];

    for bit in 0..(1 << n) {
        let mn = (0..n).find(|&i| (bit >> i) & 1 == 0).unwrap_or(n);

        for i in mn + 1..n {
            if (bit >> i) & 1 == 0 {
                let nb = bit | (1 << mn) | (1 << i);
                dp[nb] = dp[nb].max(dp[bit] + d[mn][i]);
            }
        }
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}

// 全探索による解法
pub fn solve1(n: usize, d: &Vec<Vec<usize>>) {
    let mut ans = 0;
    dfs(n, d, &mut vec![false; n], &mut ans, 0);
    println!("{}", ans);
}

fn dfs(n: usize, d: &Vec<Vec<usize>>, seen: &mut Vec<bool>, ans: &mut usize, now: usize) {
    *ans = now.max(*ans);
    let (mn, _) = (0..n).find_position(|&i| !seen[i]).unwrap_or((n, n));
    if mn == n {
        return;
    }

    seen[mn] = true;
    // 対象頂点を余らせる
    if n % 2 == 1 {
        dfs(n, d, seen, ans, now);
    }

    for v in mn + 1..n {
        if seen[v] {
            continue;
        }
        seen[v] = true;

        dfs(n, d, seen, ans, now + d[mn][v]);
        seen[v] = false;
    }
    seen[mn] = false;
}
