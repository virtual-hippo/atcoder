use ac_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }

    const ATCODER: [char; 8] = [' ', 'a', 't', 'c', 'o', 'd', 'e', 'r'];

    let mut init = [ModInt1000000007::new(0); ATCODER.len()];
    init[0] += 1;

    let dp = s.chars().fold(init, |dp, ch| {
        std::array::from_fn(|j| if j > 0 && ch == ATCODER[j] { dp[j] + dp[j - 1] } else { dp[j] })
    });
    let ans = dp[7];
    println!("{}", ans);
}
