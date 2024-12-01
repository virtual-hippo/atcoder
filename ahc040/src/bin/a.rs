#![allow(unused_imports)]
use proconio::source::line::LineSource;
use proconio::{fastout, input, marker::Chars};
use rand::Rng;
use rand_distr::num_traits::float;
use rand_distr::{Distribution, Normal};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{self, BufReader},
};

const U: char = 'U';
const L: char = 'L';
const ROTATE: u8 = 1;
const NOT_ROTATE: u8 = 0;

#[derive(Debug, Eq, PartialEq)]
pub struct Prdb {
    pub p: usize,
    pub r: u8,
    pub d: char,
    pub b: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Square {
    /// 番号
    pub i: usize,

    pub w: usize,
    pub h: usize,
}

impl Ord for Square {
    // `w` + `h` の合計 (面積で降順にソートできるようにする)
    fn cmp(&self, other: &Self) -> Ordering {
        (other.w + other.h)
            .cmp(&(self.w + self.h))
            .then_with(|| self.i.cmp(&other.i))
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct State {
    pub prdbs: Vec<Prdb>,
    pub prdbs_p: Vec<usize>,
    pub score: usize,
    pub right: usize,
    pub bottom: usize,
    pub x_tree: ahc040_lib::DynamicSegmentTree,
    pub y_tree: ahc040_lib::DynamicSegmentTree,
    pub right_each_square: Vec<usize>,
    pub bottom_each_square: Vec<usize>,
}

#[fastout]
fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    // 長方形の個数 N は30≤N≤100 を満たす。
    // 操作回数T はN/2≤T≤4N を満たす。
    // 計測時に発生する誤差の標準偏差
    // σ は1000≤σ≤10000 を満たす整数値である。
    // 横幅と縦幅の計測値wi′​,hi′​ は 1 以上 109 以下の整数値である。

    input! {
        n: usize,
        t: usize,
        sigma: usize,
        mut wh: [(usize,usize); n],
    }

    let mut squares = wh
        .iter()
        .enumerate()
        .map(|(i, (w, h))| Square { i, w: *w, h: *h })
        .collect::<Vec<Square>>();
    squares.sort();

    let mut set = HashSet::new();
    let cnt = if n < t { n } else { t - 1 };

    for i in 0..cnt {
        let prdbs = vec![Prdb {
            p: squares[i].i,
            r: 0,
            d: U,
            b: -1,
        }];
        wh[squares[i].i] = query(&prdbs);
        set.insert(squares[i].i);
    }

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let wh = {
        let mut ret = vec![];
        for i in 0..n {
            let (w, h) = wh[i];
            if set.contains(&i) {
                ret.push((w, h));
                continue;
            }

            let normal = Normal::new((w + sigma / 2) as f64, sigma as f64).expect("#");
            let w = (normal.sample(&mut rng).ceil() as usize).min(1_000_000_000);
            let normal = Normal::new((h + sigma / 2) as f64, sigma as f64).expect("#");
            let h = (normal.sample(&mut rng).ceil() as usize).min(1_000_000_000);

            ret.push((w, h));
        }
        ret
    };

    let squares = wh
        .iter()
        .enumerate()
        .map(|(i, (w, h))| Square { i, w: *w, h: *h })
        .collect::<Vec<Square>>();
    let sum_w_and_h: usize = wh.iter().map(|(w, h)| *w + *h).sum();

    let mut state = State {
        prdbs: vec![],
        prdbs_p: vec![],
        right: 0,
        bottom: 0,
        x_tree: ahc040_lib::DynamicSegmentTree::new(0, 100_000_000_000),
        y_tree: ahc040_lib::DynamicSegmentTree::new(0, 100_000_000_000),
        score: sum_w_and_h,
        right_each_square: vec![0; n],
        bottom_each_square: vec![0; n],
    };

    // 初期構築
    {
        for p in 0..n {
            if let Some((prdb, best_score)) = jikken::execute(&mut state, &squares[p], p) {
                match prdb.d {
                    U => {
                        let right = if prdb.b == -1 {
                            0
                        } else {
                            state.right_each_square[prdb.b as usize]
                        };
                        let (l, r) = (right, right + squares[p].w);
                        let now_bottom = state.x_tree.query_max(l as u64, r as u64);

                        // 更新対象区間についての更新後の下端
                        let updated_bottom = now_bottom + squares[p].h as i64;

                        state.bottom = state.bottom.max(updated_bottom as usize);
                        state.right = state.right.max(r as usize);

                        state.x_tree.update(l as u64, updated_bottom);
                        state.x_tree.update(r as u64, updated_bottom);

                        state.y_tree.update(now_bottom as u64, r as i64);
                        state.y_tree.update(updated_bottom as u64, r as i64);

                        state.right_each_square[p] = r;
                        state.bottom_each_square[p] = updated_bottom as usize;
                    }
                    L => {
                        let bottom = if prdb.b == -1 {
                            0
                        } else {
                            state.bottom_each_square[prdb.b as usize]
                        };
                        let (l, r) = (bottom, bottom + squares[p].h);
                        let now_right = state.y_tree.query_max(l as u64, r as u64);

                        // 更新対象区間についての更新後の右端
                        let updated_right = now_right + squares[p].w as i64;

                        state.right = state.right.max((updated_right) as usize);
                        state.bottom = state.bottom.max(r as usize);

                        state.y_tree.update(l as u64, updated_right);
                        state.y_tree.update(r as u64, updated_right);

                        state.x_tree.update(now_right as u64, r as i64);
                        state.x_tree.update(updated_right as u64, r as i64);

                        state.bottom_each_square[p] = r;
                        state.right_each_square[p] = updated_right as usize;
                    }
                    _ => unreachable!(),
                }

                state.prdbs.push(prdb);
                state.prdbs_p.push(p);
                state.score = best_score;
            }
        }
    }

    for _ in 0..(t - cnt) {
        query(&state.prdbs);
    }
}

mod jikken {
    use super::*;

    pub fn execute(state: &mut State, square: &Square, p: usize) -> Option<(Prdb, usize)> {
        let mut best_score = state.score;
        let mut prdb = None;

        for r in [ROTATE, NOT_ROTATE] {
            for d in [U, L] {
                for prdbs_pi in 0..state.prdbs_p.len() {
                    let score = if r == ROTATE {
                        estimate(state, square.h, square.w, d, state.prdbs_p[prdbs_pi] as i64)
                    } else {
                        estimate(state, square.w, square.h, d, state.prdbs_p[prdbs_pi] as i64)
                    };

                    if best_score >= score {
                        best_score = score;
                        prdb = Some(Prdb {
                            p,
                            r,
                            d,
                            b: state.prdbs_p[prdbs_pi] as i64,
                        });
                    }
                }

                // b = -1 のとき
                {
                    let score = if r == 0 {
                        estimate(state, square.w, square.h, d, -1)
                    } else {
                        estimate(state, square.h, square.w, d, -1)
                    };

                    if best_score >= score {
                        best_score = score;
                        prdb = Some(Prdb { p, r, d, b: -1 });
                    }
                }
            }
        }

        if let Some(prdb) = prdb {
            Some((prdb, best_score))
        } else {
            None
        }
    }

    fn estimate(state: &mut State, w: usize, h: usize, d: char, b: i64) -> usize {
        match d {
            U => {
                let right = if b == -1 {
                    0
                } else {
                    state.right_each_square[b as usize]
                };
                estimate_u(state, w, h, right)
            }
            L => {
                let bottom = if b == -1 {
                    0
                } else {
                    state.bottom_each_square[b as usize]
                };
                estimate_l(state, w, h, bottom)
            }
            _ => unreachable!(),
        }
    }

    fn estimate_u(state: &mut State, w: usize, h: usize, right: usize) -> usize {
        let (l, r) = (right, right + w);
        let now_bottom = state.x_tree.query_max(l as u64, r as u64);

        // 更新対象区間についての更新後の下端
        let updated_bottom = now_bottom + h as i64;

        let diff_h = state.bottom.max(updated_bottom as usize) - state.bottom;
        let diff_w = state.right.max(r as usize) - state.right;

        state.score + diff_h + diff_w - w - h
    }

    fn estimate_l(state: &mut State, w: usize, h: usize, bottom: usize) -> usize {
        let (l, r) = (bottom, bottom + h);
        let now_right = state.y_tree.query_max(l as u64, r as u64);

        // 更新対象区間についての更新後の右端
        let updated_right = now_right + w as i64;

        let diff_w = state.right.max((updated_right) as usize) - state.right;
        let diff_h = state.bottom.max(r as usize) - state.bottom;

        state.score + diff_w + diff_h - w - h
    }
}

fn query(prdbs: &Vec<Prdb>) -> (usize, usize) {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    println!("{}", prdbs.len());
    for prdb in prdbs.iter() {
        println!("{} {} {} {}", prdb.p, prdb.r, prdb.d, prdb.b);
    }
    input! {
        w: usize,
        h: usize,
    }
    return (w, h);
}

mod ahc040_lib {
    use std::cell::RefCell;
    use std::cmp::{max, min};
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct DynamicSegmentTree {
        start: u64,
        end: u64,
        sum: i64,
        max: i64,
        left: Option<Rc<RefCell<DynamicSegmentTree>>>,
        right: Option<Rc<RefCell<DynamicSegmentTree>>>,
    }

    impl DynamicSegmentTree {
        pub fn new(start: u64, end: u64) -> Self {
            DynamicSegmentTree {
                start,
                end,
                sum: 0,
                max: 0,
                left: None,
                right: None,
            }
        }

        // 更新操作
        pub fn update(&mut self, idx: u64, val: i64) {
            if self.start == self.end {
                self.sum = val;
                self.max = val;
                return;
            }

            let mid = (self.start + self.end) / 2;
            if idx <= mid {
                if self.left.is_none() {
                    self.left = Some(Rc::new(RefCell::new(DynamicSegmentTree::new(
                        self.start, mid,
                    ))));
                }
                self.left.as_ref().unwrap().borrow_mut().update(idx, val);
            } else {
                if self.right.is_none() {
                    self.right = Some(Rc::new(RefCell::new(DynamicSegmentTree::new(
                        mid + 1,
                        self.end,
                    ))));
                }
                self.right.as_ref().unwrap().borrow_mut().update(idx, val);
            }

            self.sum = self.left.as_ref().map_or(0, |left| left.borrow().sum)
                + self.right.as_ref().map_or(0, |right| right.borrow().sum);
            self.max = max(
                self.left.as_ref().map_or(0, |left| left.borrow().max),
                self.right.as_ref().map_or(0, |right| right.borrow().max),
            );
        }

        // クエリ操作
        pub fn query_max(&self, l: u64, r: u64) -> i64 {
            if l > self.end || r < self.start {
                return 0;
            }
            if l <= self.start && r >= self.end {
                return self.max;
            }

            let left_max = self
                .left
                .as_ref()
                .map_or(0, |left| left.borrow().query_max(l, r));
            let right_max = self
                .right
                .as_ref()
                .map_or(0, |right| right.borrow().query_max(l, r));
            max(left_max, right_max)
        }
    }
}
