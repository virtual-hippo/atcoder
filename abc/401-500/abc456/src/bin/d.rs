use ac_library::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let to_usize = |ch: char| (ch as u8 - b'a') as usize;

    let dp = (0..n).fold(
        [ModInt998244353::new(0), ModInt998244353::new(0), ModInt998244353::new(0)],
        |mut dp, i| {
            let j = to_usize(s[i]);
            dp[j] = dp[to_usize('a')] + dp[to_usize('b')] + dp[to_usize('c')] + 1;
            dp
        },
    );

    let ans = dp[to_usize('a')] + dp[to_usize('b')] + dp[to_usize('c')];
    println!("{}", ans);
}
