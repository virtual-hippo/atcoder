// list: [0: val, 1: val, 2: val, 3: val, 4: val, 5: val, 6: val]
// if index <= 3 is ok return  (3, 4)
pub fn binary_search<F: Fn(i64) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let target = 15;
        let list = vec![3, 5, 7, 10, 15, 18];
        let is_ok = |x: i64| list[x as usize] <= target;
        let result = binary_search((0, list.len() as i64), is_ok);
        assert_eq!(result, (4, 5));
    }
    #[test]
    fn it_works2() {
        let target = 21;
        let list = vec![3, 5, 7, 10, 15, 18];
        let is_ok = |x: i64| list[x as usize] <= target;
        let result = binary_search((0, list.len() as i64), is_ok);
        assert_eq!(result, (5, 6));
    }
}
