#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let ss = {
        let mut ret = vec![0; 3 * n + 1];
        for i in 0..3 * n {
            ret[i + 1] = a[i % n] + ret[i];
        }
        ret
    };
    if ss[n] == s {
        println!("Yes");
        return;
    }

    let s = if s <= ss[n] { s } else { (s % ss[n]) + ss[n] };

    for l in 0..ss.len() - 1 {
        let f = |r| ss[r as usize] - ss[l] <= s;
        let (r, _) = binary_search(((l + 1) as i64, ss.len() as i64), f);

        let r = r as usize;
        if ss[r] - ss[l] == s {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

pub fn binary_search<F: Fn(i64) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
