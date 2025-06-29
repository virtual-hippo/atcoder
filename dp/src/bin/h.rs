use ac_library::ModInt1000000007;
use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    type MInt = ModInt1000000007;

    let mut dp = [[MInt::new(0); 1010]; 1010];
    dp[0][0] = MInt::new(1);

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; 1010]; 1010];
    visited[0][0] = true;
    queue.push_back((0, 0));

    while let Some((i, j)) = queue.pop_front() {
        if i + 1 < h && a[i + 1][j] == '.' {
            dp[i + 1][j] += dp[i][j];
            if !visited[i + 1][j] {
                visited[i + 1][j] = true;
                queue.push_back((i + 1, j));
            }
        }
        if j + 1 < w && a[i][j + 1] == '.' {
            dp[i][j + 1] += dp[i][j];
            if !visited[i][j + 1] {
                visited[i][j + 1] = true;
                queue.push_back((i, j + 1));
            }
        }
    }

    let ans = dp[h - 1][w - 1];
    println!("{}", ans);
}
