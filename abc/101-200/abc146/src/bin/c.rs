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
        a: usize,
        b: usize,
        x: usize,
    }
    let d = |i: usize| -> usize { i.to_string().len() };
    let is_ok = |i: usize| a * i + b * d(i) <= x;
    let result = binary_search((0, 1_000_000_001), is_ok);
    println!("{}", result.0);
}
