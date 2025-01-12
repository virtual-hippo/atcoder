#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        start: (usize, usize),
        n: usize,
    }

    let (kabe_r, kabe_c) = {
        let mut kabe_r = FxHashMap::default();
        let mut kabe_c = FxHashMap::default();
        for _ in 0..n {
            input! {
                (r,c): (usize, usize),
            }
            kabe_r.entry(r).or_insert_with(|| vec![]).push(c);
            kabe_c.entry(c).or_insert_with(|| vec![]).push(r);
        }
        kabe_r.iter_mut().for_each(|(_, v)| v.sort());
        kabe_c.iter_mut().for_each(|(_, v)| v.sort());
        (kabe_r, kabe_c)
    };

    input! {
        q: usize,
    }

    let mut now = start;

    for _ in 0..q {
        input! {
            d: char,
            l: usize,
        }

        match d {
            'L' => {
                let rr = now.1;
                if let Some(kabe) = kabe_r.get(&now.0) {
                    let ind = kabe.upper_bound(&rr);
                    let ind = ind.saturating_sub(1);
                    if ind == 0 && rr < kabe[ind] {
                        now.1 = 1.max(now.1.saturating_sub(l));
                    } else if kabe[ind] < rr {
                        now.1 = 1.max(kabe[ind] + 1).max(now.1.saturating_sub(l));
                    }
                } else {
                    now.1 = 1.max(now.1.saturating_sub(l));
                }
            }
            'R' => {
                let rr = now.1;
                if let Some(kabe) = kabe_r.get(&now.0) {
                    let ind = kabe.upper_bound(&rr);
                    if ind == kabe.len() {
                        now.1 = w.min(now.1 + l);
                    } else if rr < kabe[ind] {
                        now.1 = w.min(kabe[ind] - 1).min(now.1 + l);
                    }
                } else {
                    now.1 = w.min(now.1 + l);
                }
            }
            'U' => {
                let cc = now.0;
                if let Some(kabe) = kabe_c.get(&now.1) {
                    let ind = kabe.upper_bound(&cc);
                    let ind = ind.saturating_sub(1);
                    if ind == 0 && cc < kabe[ind] {
                        now.0 = 1.max(now.0.saturating_sub(l));
                    } else if kabe[ind] < cc {
                        now.0 = 1.max(kabe[ind] + 1).max(now.0.saturating_sub(l));
                    }
                } else {
                    now.0 = 1.max(now.0.saturating_sub(l));
                }
            }
            'D' => {
                let cc = now.0;
                if let Some(kabe) = kabe_c.get(&now.1) {
                    let ind = kabe.upper_bound(&cc);
                    if ind == kabe.len() {
                        now.0 = h.min(now.0 + l);
                    } else if cc < kabe[ind] {
                        now.0 = h.min(kabe[ind] - 1).min(now.0 + l);
                    }
                } else {
                    now.0 = h.min(now.0 + l);
                }
            }
            _ => unreachable!(),
        }
        println!("{} {}", now.0, now.1);
    }
}
