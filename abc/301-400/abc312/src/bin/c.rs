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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let is_ok = |mid: usize| {
        let na = a.iter().filter(|&&v| v <= mid).count();
        let nb = b.iter().filter(|&&v| v >= mid).count();
        na < nb
    };
    let (_, ans) = binary_search((0, 1_000_000_001), is_ok);
    println!("{}", ans);
}
