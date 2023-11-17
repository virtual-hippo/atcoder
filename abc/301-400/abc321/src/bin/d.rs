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
        p: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();

    let mut sb = vec![0; m + 1];
    for i in 0..m {
        sb[i + 1] = sb[i] + b[i];
    }

    let mut ans = 0;

    for i in 0..n {
        if a[i] >= p {
            ans += p * m;
            continue;
        }

        let is_ok = |x: usize| a[i] + b[x] <= p;
        let (pos, _) = binary_search((0, b.len()), is_ok);
        if a[i] + b[pos] <= p {
            ans += sb[pos + 1] + a[i] * (pos + 1);
            ans += p * (m - (pos + 1));
        } else {
            ans += p * m;
        }
    }

    println!("{}", ans);
}
