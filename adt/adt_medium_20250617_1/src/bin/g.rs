use proconio::{fastout, input};

fn gcd(x: u64, y: u64) -> u64 {
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
fn lcm(x: u64, y: u64) -> u64 {
    let d = gcd(x, y);
    x / d * y
}

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64,
    }

    let (n, m) = if n < m { (n, m) } else { (m, n) };
    let l = lcm(n, m);

    let is_ok = |x| x / n + x / m - 2 * (x / l) < k;
    let (_, ans) = binary_search((0, u64::MAX), is_ok);

    println!("{}", ans);
}

pub fn binary_search<F: Fn(u64) -> bool>(initial_pos: (u64, u64), is_ok: F) -> (u64, u64) {
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
