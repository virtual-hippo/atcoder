#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::*;
use superslice::Ext;

type MInt = ModInt998244353;

pub struct Sieve {
    /// 最小の素因数
    f: Vec<usize>,
    /// 素数一覧
    pub primes: Vec<usize>,
}

impl Sieve {
    pub fn new(n: usize) -> Self {
        let mut f = vec![0; n + 1];
        let mut primes = vec![];

        f[0] = 1;
        f[1] = 1;

        for x in 2..=n {
            if f[x] != 0 {
                continue;
            }
            primes.push(x);
            f[x] = x;

            // x の倍数にマーキングする
            ((x * x)..).step_by(x).take_while(|&y| y < n + 1).for_each(|y| {
                if f[y] == 0 {
                    f[y] = x;
                }
            });
        }

        Self { f, primes }
    }

    pub fn is_prime(&self, x: usize) -> bool {
        1 < x && self.f[x] == x
    }

    pub fn factor_list(&self, mut x: usize) -> Vec<usize> {
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.f[x]);
            x /= self.f[x];
        }
        factors
    }

    pub fn factor(&self, x: usize) -> Vec<(usize, usize)> {
        self.factor_list(x)
            .iter()
            .copied()
            .dedup_with_count()
            .map(|(count, prime)| (prime, count)) // (素因数, 指数) の順に揃える
            .collect()
    }
}

fn solve(sieve: &Sieve) {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    for i in 0..n {
        sieve.factor(a[i]).iter().for_each(|&(p, e)| {
            map.entry(p).or_insert_with(|| vec![]).push(e);
        });
    }

    let mut base = MInt::new(1);
    for (&p, xs) in map.iter_mut() {
        xs.push(0);
        xs.sort_by_key(|&x| std::cmp::Reverse(x));
        base *= MInt::new(p).pow(xs[0] as u64);
    }

    let get_mx = |a: &[usize], x: usize| -> usize {
        if x == a[0] {
            a[0] - a[1]
        } else {
            0
        }
    };

    let ans = (0..n)
        .map(|i| {
            let mut now = base;
            sieve.factor(a[i]).iter().for_each(|&(p, e)| {
                now /= MInt::new(p).pow(get_mx(&map[&p], e) as u64);
            });
            now
        })
        .collect_vec();
    print_vec_1line(&ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    let sieve = Sieve::new(10_000_010);

    for _ in 0..t {
        solve(&sieve);
    }
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
