///
/// 入力された正整数の各桁を要素に持つ Vec を作成する
/// ```
/// use abclib::utils::digits_to_vec;
/// let ret = digits_to_vec(12345);
/// assert_eq!(ret, vec![1, 2, 3, 4, 5]);
/// ```
pub fn digits_to_vec(x: usize) -> Vec<usize> {
    if x / 10 == 0 {
        vec![x % 10]
    } else {
        digits_to_vec(x / 10).into_iter().chain(std::iter::once(x % 10)).collect()
    }
}

///
/// 桁数を計算
/// ```
/// use abclib::utils::num_digits;
/// assert_eq!(num_digits(0), 1);
/// assert_eq!(num_digits(5), 1);
/// assert_eq!(num_digits(125), 3);
/// assert_eq!(num_digits(12345), 5);
/// assert_eq!(num_digits(123456789012345), 15);
/// ```
pub fn num_digits(x: u64) -> usize {
    if x == 0 {
        1
    } else {
        x.ilog10() as usize + 1
    }
}
