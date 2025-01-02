#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        _: usize,
        m: usize,
    }
    let mut row_w_map = FxHashMap::default();
    let mut row_w = vec![];

    let mut col_w_map = FxHashMap::default();
    let mut col_w = vec![];

    let mut blacks = vec![];

    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
            c: char,
        }
        match c {
            'B' => {
                blacks.push((x, y));
            }
            'W' => {
                if let Some(py) = row_w_map.get_mut(&x) {
                    *py = y.min(*py);
                } else {
                    row_w_map.insert(x, y);
                }

                if let Some(px) = col_w_map.get_mut(&y) {
                    *px = x.min(*px);
                } else {
                    col_w_map.insert(y, x);
                }

                row_w.push(x);
                col_w.push(y);
            }
            _ => unreachable!(),
        };
    }

    row_w.sort();
    col_w.sort();

    for (bx, by) in blacks.iter() {
        if !f(*bx, &row_w, *by, &row_w_map) {
            println!("No");
            return;
        }
        if !f(*by, &col_w, *bx, &col_w_map) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn f(b: usize, w_vec: &Vec<usize>, right: usize, map: &FxHashMap<usize, usize>) -> bool {
    let is_ok = |x: i64| w_vec[x as usize] < b;
    let (wi, _) = binary_search((-1, w_vec.len() as i64), is_ok);

    if wi == w_vec.len() as i64 {
        return true;
    }
    if wi == -1 {
        return true;
    }

    let wi = wi as usize;

    if let Some(w) = map.get(&w_vec[wi]) {
        if *w <= right {
            return false;
        }
    }

    true
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
