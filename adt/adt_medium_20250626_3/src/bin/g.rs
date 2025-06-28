use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        l: [u64; n],
    }

    let mx_l = l.iter().max().unwrap();
    let sum_l = l.iter().map(|&v| v + 1).sum::<u64>();

    let calc_m = |w: u64| -> u64 {
        let mut now = l[0];
        let mut m = 1;

        for i in 1..n {
            now += 1 + l[i];
            if now > w {
                now = l[i];
                m += 1;
            }
        }
        m
    };

    let is_ok = |w: u64| -> bool { m < calc_m(w) };
    let ans = binary_search((mx_l - 1, sum_l + 1), is_ok).1;

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
