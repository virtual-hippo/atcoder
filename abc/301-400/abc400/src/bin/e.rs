#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

///! エラトステネスの篩
///! https://algo-method.com/descriptions/64
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    // 素数がtrueとなったベクタ
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i] == false {
            continue;
        }
        let mut j = i * 2;
        while j < n + 1 {
            is_prime[j] = false;
            j += i;
        }
    }
    is_prime
}

// 平方数であることに着目した解法
fn solve() {
    let n = 1_000_001;
    // 1_000_001 以下の整数について, それぞれの素因数の数を求める
    let cnt = {
        let mut cnt = vec![0_usize; n + 1];

        for i in 2..n + 1 {
            if cnt[i] > 0 {
                continue;
            }
            let mut j = i;
            while j < n + 1 {
                cnt[j] += 1;
                j += i;
            }
        }
        cnt
    };

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: f64,
        }
        let mut b = a.sqrt() as usize;
        while cnt[b] != 2 {
            b -= 1;
        }
        println!("{}", b * b);
    }
}

// 素数の組み合わせを列挙する解法
fn solve2() {
    let is_prime = sieve_of_eratosthenes(1_000_001);
    let primes_p = is_prime
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i as u64)
        .filter(|&i| i <= 1_000)
        .collect::<Vec<u64>>();
    let primes_q = is_prime
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i as u64)
        .filter(|&i| i <= 1_000_000)
        .collect::<Vec<u64>>();

    let mut candidate = vec![];
    for (i, &p) in primes_p.iter().enumerate() {
        for &q in primes_q.iter().skip(i + 1) {
            if p * q > 1_000_000 {
                break;
            }

            let mut pp = p;
            while pp <= 1_000_000 {
                let mut qq = q;
                while qq <= 1_000_000 {
                    let m = pp * qq;
                    if m <= 1000_000 {
                        candidate.push(m * m);
                    }
                    qq *= q;
                }
                pp *= p;
            }
        }
    }
    candidate.sort();

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: u64,
        }
        let l = candidate.upper_bound(&a) - 1;
        println!("{}", candidate[l]);
    }
}

#[fastout]
fn main() {
    solve();
}
