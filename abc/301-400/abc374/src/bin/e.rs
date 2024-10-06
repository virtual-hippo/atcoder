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
        x: usize,
    }
    let mut s = vec![];
    let mut t = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            p: usize,
            b: usize,
            q: usize,
        }
        if b * p < a * q {
            s.push((a, p));
            t.push((b, q));
        } else {
            s.push((b, q));
            t.push((a, p));
        }
    }

    let is_ok = |m: usize| -> bool {
        let mut cost = 0;

        for i in 0..n {
            let mut now = usize::MAX;
            for j in 0..s[i].0 {
                let t_cnt = t[i].0 * j;
                let nokori = if t_cnt <= m { m - t_cnt } else { 0 };
                let k = if nokori % s[i].0 == 0 {
                    nokori / s[i].0
                } else {
                    nokori / s[i].0 + 1
                };
                now = now.min(t[i].1 * j + s[i].1 * k);
            }
            cost += now;
            if cost > x {
                return false;
            }
        }
        cost <= x
    };

    let result = binary_search((0, 1_000_000_002), is_ok);
    let ans = result.0;
    println!("{}", ans);
}
