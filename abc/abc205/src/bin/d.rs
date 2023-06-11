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
        q: usize,
        a: [u64; n],
    }
    let mut c = vec![0; n+1];
    c[0] = a[0] - 1;
    for i in 1..n {
        c[i] = c[i-1] + a[i] - a[i-1] - 1;
    }
    for _ in 0..q {
        input! {
            k: u64,
        }
        if c[n-1] < k {
            println!("{}", a[n-1] + k - c[n-1]);
        } else {
            let is_ok = |x: usize| c[x] < k;
            let result = binary_search((0, c.len()), is_ok);
            println!("{}", a[result.1] - a[result.1-1] + 1);
        }
    }
}

