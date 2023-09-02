use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    // 各選挙区について何議席足りないか求める
    let mut diff = Vec::with_capacity(n);
    let (mut takahashi_giseki, mut aoki_giseki) = (0, 0);
    for _ in 0..n {
        input! {
            (x,y,z): (i64, i64, i64),
        }

        if x > y {
            takahashi_giseki += z;
        } else {
            aoki_giseki += z;
            let kahansu = (x + y) / 2 + 1;
            diff.push((kahansu - x, z));
        }
    }

    if takahashi_giseki > aoki_giseki {
        println!("{}", 0);
        return;
    }

    let sougiseki = takahashi_giseki + aoki_giseki;

    // 部分和DP
    let mut dp = vec![vec![-1; (sougiseki + 1) as usize]; diff.len() + 1];
    dp[0][0] = 0;
    for i in 1..diff.len() + 1 {
        for j in 0..dp[0].len() {
            if dp[i - 1][j] != -1 {
                let new_val = dp[i - 1][j];
                if dp[i][j] == - 1{
                    dp[i][j] = new_val;
                } else {
                    dp[i][j] = dp[i][j].min(new_val)
                }
            }
            if j < diff[i - 1].1 as usize {
                // 何もしない
            } else if dp[i - 1][j - (diff[i - 1].1 as usize)] != -1 {
                let new_val = dp[i - 1][j - (diff[i - 1].1 as usize)] + diff[i - 1].0;
                if dp[i][j] == - 1{
                    dp[i][j] = new_val;
                } else {
                    dp[i][j] = dp[i][j].min(new_val)
                }
            }
        }
    }

    // 何議席とればよいか
    let atonanngiseki = ((sougiseki / 2 + 1) - takahashi_giseki) as usize;

    let mut ans = i64::MAX;
    for i in 0..diff.len() + 1 {
        for j in atonanngiseki..dp[0].len() {
            if dp[i][j as usize] != -1 {
                ans = ans.min(dp[i][j as usize]);
            }
        }
    }
    println!("{}", ans);
}
