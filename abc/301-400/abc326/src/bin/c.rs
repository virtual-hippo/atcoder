use proconio::input;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();

    let mut ans = 0;
    for i in 0..n {
        let target = a[i] + m;
        let is_ok = |x: usize| a[x] < target;
        let result = binary_search((0, a.len()), is_ok);
        ans = ans.max((result.0 + 1) - i);
    }
    println!("{}", ans);
}
