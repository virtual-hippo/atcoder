use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: u64,
    }

    let max_y = {
        let mut y = 1;
        while y * y <= d {
            y += 1;
        }
        y
    };

    let mut ans = u64::MAX;
    let mut x = 0;
    while x * x <= d {
        let is_ok = |y: u64| -> bool { y * y <= d - x * x };
        let (y, y1) = binary_search((0, max_y), is_ok);

        let now = (x * x + y * y).abs_diff(d);
        ans = ans.min(now);

        let y = y1;
        let now = (x * x + y * y).abs_diff(d);
        ans = ans.min(now);

        x += 1;
    }

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
