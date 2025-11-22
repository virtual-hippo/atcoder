use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let acm = {
        let mut aa = vec![0; n];
        let mut bb = vec![0; n];

        for i in 0..n {
            if s[i] == 'a' {
                aa[i] = 1;
            }
            if s[i] == 'b' {
                bb[i] = 1;
            }
        }

        let mut acm = vec![(0, 0)];
        for i in 0..n {
            let na = acm[i].0 + aa[i];
            let nb = acm[i].1 + bb[i];
            acm.push((na, nb));
        }
        acm
    };
    let mut ans = 0_u64;

    for l in 0..n {
        let is_ok_a = |r: usize| {
            let cnt_a = acm[r].0 - acm[l].0;
            cnt_a < a
        };
        let a_r = binary_search((l, n + 1), is_ok_a).1;

        let is_ok_b = |r: usize| {
            let cnt_b = acm[r].1 - acm[l].1;
            cnt_b < b
        };
        let b_r = binary_search((l, n + 1), is_ok_b).1;

        if b_r > a_r {
            ans += (b_r - a_r) as u64;
        }
    }

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
