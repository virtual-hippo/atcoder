use proconio::{fastout, input};

fn gcd(x: usize, y: usize) -> usize {
    let mut xy = (x, y);
    while xy.0 >= 1 && xy.1 >= 1 {
        if xy.0 < xy.1 {
            xy.1 %= xy.0;
        } else {
            xy.0 %= xy.1;
        }
    }
    if xy.0 >= 1 {
        xy.0
    } else {
        xy.1
    }
}

fn lcm(x: usize, y: usize) -> usize {
    let d = gcd(x, y);
    x / d * y
}

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
        m: usize,
        k: usize,
    }
    let max: usize = 2 * 1_000_000_000_000_000_000;
    let l = lcm(n, m);
    let is_ok = |x: usize| (x / n) + (x / m) - 2 * (x / l) < k;
    let result = binary_search((0, max), is_ok);
    print!("{}", result.1);
}
