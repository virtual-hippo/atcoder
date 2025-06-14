use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![None; h + 1]; n + 1];
    dp[0][h] = Some(m);

    for i in 0..n {
        let (a, b) = ab[i];

        for j in 0..h + 1 {
            if let Some(b_now) = dp[i][j] {
                if b_now >= b {
                    if let Some(b_new) = dp[i + 1][j] {
                        let v = (b_now - b).max(b_new);
                        dp[i + 1][j] = Some(v);
                    } else {
                        dp[i + 1][j] = Some(b_now - b);
                    }
                }

                if j < a {
                    continue;
                }

                if let Some(b_new) = dp[i + 1][j - a] {
                    let v = b_now.max(b_new);
                    dp[i + 1][j - a] = Some(v);
                } else {
                    dp[i + 1][j - a] = Some(b_now);
                }
            }
        }
    }

    for i in 1..=n {
        let is_ok = dp[i].iter().any(|&x| x.is_some());

        if !is_ok {
            println!("{}", i - 1);
            return;
        }
    }

    println!("{}", n);
}
