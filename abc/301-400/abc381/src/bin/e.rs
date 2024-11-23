use proconio::{fastout, input, marker::Chars};
use superslice::Ext;

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid as usize) {
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
        q: usize,
        s: Chars,
    }
    let is = {
        let mut ret = vec![vec![]; 3];
        for i in 0..n {
            let c = if s[i] == '/' {
                0_usize
            } else {
                (s[i] as u8 - b'0') as usize
            };
            ret[c].push(i);
        }
        ret
    };

    let get_next = |c: usize, i: usize, x: usize| {
        if x == 0 {
            return i;
        }

        let nis = &is[c];
        let j = nis.lower_bound(&i);
        let j = j + (x - 1);
        if j < nis.len() {
            nis[j] + 1
        } else {
            usize::MAX
        }
    };

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let l = l - 1;

        let is_ok = |k| {
            let i = l;
            let i = get_next(1, i, k);
            let i = get_next(0, i, 1);
            let i = get_next(2, i, k);
            i <= r
        };
        let result = binary_search((-1, n as i64), is_ok);

        let ans = if result.0 == -1 { 0 } else { result.0 * 2 + 1 };
        println!("{}", ans);
    }
}
