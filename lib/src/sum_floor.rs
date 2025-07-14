///
/// (0..n).map(|i| n / i).sum() を O(√N) で求める
/// # Reference
/// - ABC414-E
///     - https://www.youtube.com/watch?v=rLoeRm2BXDk
///     - https://atcoder.jp/contests/abc414/editorial/13450
/// - ABC230-E
///     - https://atcoder.jp/contests/abc230/editorial/3015
///     - https://atcoder.jp/contests/abc230/editorial/5127
///
pub mod sum_floor {
    use itertools::Itertools;

    pub fn sum_floor(n: u64) -> u64 {
        std::iter::successors(Some(1), |&i| {
            if i <= n {
                let y = n / i;
                Some(n / y + 1)
            } else {
                None
            }
        })
        .tuple_windows()
        .map(|(i, ni)| {
            let y = n / i;
            (ni - i) * y
        })
        .sum()
    }
}
