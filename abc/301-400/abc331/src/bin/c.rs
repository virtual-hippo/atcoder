use proconio::{fastout, input};
pub fn binary_search<F: Fn(usize) -> bool>(
    initial_pos: (usize, usize),
    is_ok: F,
) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left, right)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut sorted_a = a.clone();
    sorted_a.sort();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + sorted_a[i];
    }
    for i in 0..n {
        let is_ok = |x: usize| sorted_a[x] <= a[i];
        let result = binary_search((0, sorted_a.len()), is_ok);
        let ans = s[n] - s[result.1];
        print!("{} ", ans);
    }
}
