#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

fn rc(d: char) -> (i64, i64) {
    let di = [-1, 0, 1, 0];
    let dj = [0, -1, 0, 1];
    match d {
        'U' => (di[0], dj[0]),
        'L' => (di[1], dj[1]),
        'D' => (di[2], dj[2]),
        'R' => (di[3], dj[3]),
        _ => unreachable!(),
    }
}

// ax + b = cx + d を解く
// 解なし: None, 常に成立: Some(i64::MAX), 具体的な解: Some(x)
fn calc(a: i64, b: i64, c: i64, d: i64) -> Option<i64> {
    let mut l = a - c;
    let mut r = d - b;
    if l == 0 {
        return if r == 0 { Some(i64::MAX) } else { None };
    }

    if l * r < 0 {
        return None;
    }

    if l < 0 {
        l = -l;
        r = -r;
    }
    if r % l != 0 {
        return None;
    }
    Some(r / l)
}

fn count(t: (i64, i64), a: (i64, i64), dt: (i64, i64), da: (i64, i64), w: i64) -> u64 {
    let rx = calc(dt.0, t.0, da.0, a.0);
    let cx = calc(dt.1, t.1, da.1, a.1);

    match (rx, cx) {
        (None, _) | (_, None) => 0,
        // 両方 INF: 常に一致
        (Some(i64::MAX), Some(i64::MAX)) => w as u64,
        // 片方が INF、もう片方が具体的な解
        (Some(i64::MAX), Some(x)) | (Some(x), Some(i64::MAX)) => {
            if 0 < x && x <= w {
                1
            } else {
                0
            }
        },
        // 両方具体的な解
        (Some(rx), Some(cx)) => {
            if rx == cx && 0 < rx && rx <= w {
                1
            } else {
                0
            }
        },
    }
}

#[fastout]
fn main() {
    input! {
        mut takahashi: (i64, i64),
        mut aoki: (i64, i64),
        n: u64,
        m: usize,
        l: usize,
        mut s: [(char, i64); m],
        mut t: [(char, i64); l],
    }

    let mut pi = 0;
    let mut pj = 0;

    let mut tt = 0;

    let mut ans = 0;

    while tt < n {
        let x = s[pi].1.min(t[pj].1);
        let dt = rc(s[pi].0);
        let da = rc(t[pj].0);
        ans += count(takahashi, aoki, dt, da, x);

        {
            let (di, dj) = dt;
            takahashi.0 += di * x;
            takahashi.1 += dj * x;
        }

        {
            let (di, dj) = da;
            aoki.0 += di * x;
            aoki.1 += dj * x;
        }
        tt += x as u64;

        if s[pi].1 < t[pj].1 {
            t[pj].1 -= s[pi].1;
            pi += 1;
        } else if t[pj].1 < s[pi].1 {
            s[pi].1 -= t[pj].1;
            pj += 1;
        } else {
            pi += 1;
            pj += 1;
        }
    }

    println!("{}", ans);
}
