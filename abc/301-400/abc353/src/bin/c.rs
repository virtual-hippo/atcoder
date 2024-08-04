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
        n: usize, // 2 <= n <= 300_000
        mut a: [usize; n],
    }
    let m = 100_000_000;
    a.sort();

    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] % m) * ((n - 1) % m);
    }

    let mut cnt = 0;
    for i in 0..n - 1 {
        let f = |x| a[x] + a[i] < m;
        let result = binary_search((i, n), f);
        let now = n - result.0 - 1;
        cnt += now;
    }
    ans -= cnt * m;
    println!("{}", ans);
}
