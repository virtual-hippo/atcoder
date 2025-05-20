use proconio::{fastout, input};

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

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let calc = |x: u64| -> f64 {
        let v0 = x as f64 * b as f64;
        let v1 = (a as f64 * ((x + 1) as f64).sqrt()) / (x + 1) as f64;

        v0 + v1
    };

    let is_ok = |x: u64| {
        let pre = calc(x - 1);
        let now = calc(x);
        pre > now
    };
    let (left, _) = binary_search((1, 1_000_000_000_000_000_100), is_ok);

    let ans = calc(left).min(calc(0));
    println!("{}", ans);
}
