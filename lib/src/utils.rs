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
