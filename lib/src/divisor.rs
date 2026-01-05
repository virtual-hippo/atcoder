use itertools::*;

/// 約数一覧を取得
///
/// ```
/// use abclib::divisor::get_divisors;
///
/// assert_eq!(get_divisors(6), vec![1,2,3,6]);
/// ```
pub fn get_divisors(n: usize) -> Vec<usize> {
    (1_usize..)
        .take_while(|&x| x.pow(2) <= n)
        .filter(|&x| n % x == 0)
        .map(|x| if x * x == n { vec![x] } else { vec![x, n / x] })
        .flatten()
        .sorted()
        .collect_vec()
}
