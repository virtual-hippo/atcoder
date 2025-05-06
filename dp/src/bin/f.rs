use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let sn = s.len();
    let tn = t.len();

    let mut dp = vec![vec![0; tn + 1]; sn + 1];

    for i in 0..sn + 1 {
        for j in 0..tn + 1 {
            if i > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            }

            if j > 0 {
                dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            }

            if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }
    let (mut i, mut j) = (sn, tn);
    let mut ans = vec![];
    while i > 0 && j > 0 {
        if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
            continue;
        }
        if i > 0 && j > 0 {
            if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
            continue;
        }
        if i > 0 {
            i -= 1;
            continue;
        }
        if j > 0 {
            j -= 1;
            continue;
        }
    }

    let ans = ans.iter().rev().collect::<String>();

    println!("{}", ans);
}
