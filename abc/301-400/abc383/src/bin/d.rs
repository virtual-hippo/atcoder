#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let m = 2_000_000;
    let mut set = HashSet::new();

    let mmm = sieve_of_eratosthenes(m);
    let sosu: Vec<usize> = mmm
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i)
        .collect();
    let len = sosu.len();
    for i in 0..len {
        let v = ((sosu[i].pow(2)).pow(2)).pow(2);
        if v > n {
            break;
        }
        set.insert(v);
    }

    for i in 0..len {
        for j in i + 1..len {
            if sosu[i].pow(2) > n / (sosu[j].pow(2)) {
                break;
            }
            let v = sosu[i].pow(2) * sosu[j].pow(2);
            if v > n {
                break;
            }
            let v = sosu[i].pow(2) * sosu[j].pow(2);
            if v > n {
                break;
            }
            set.insert(v);
        }
    }

    let ans = set.len();
    println!("{}", ans);
}

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
