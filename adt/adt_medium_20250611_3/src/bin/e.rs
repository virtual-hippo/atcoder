use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>();
    if sum <= m {
        println!("infinite");
        return;
    }

    let is_ok = |x: usize| {
        let s = a.iter().map(|&v| v.min(x)).sum::<usize>();
        s <= m
    };

    let ans = binary_search((0, 200_000_000_000_100), is_ok).0;
    println!("{}", ans);
}

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (usize, usize), is_ok: F) -> (usize, usize) {
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
