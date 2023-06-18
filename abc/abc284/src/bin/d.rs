// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

pub fn prime_factorize(input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i * i <= target {
        if target % i != 0 {
            i += 1;
            continue;
        }
        let mut e = 0;
        while target % i == 0 {
            e += 1;
            target /= i;
        }
        res.push((i, e));

        i += 1;
    }
    if target != 1 {
        res.push((target, 1));
    }
    res
}

fn solve() {
    input! {
        n: usize,
    }
    let mut i = 2;
    while i * i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        } else {
            break;
        }
    }
    let koho = (i, n / i);
    let (p, q)= if koho.1 % koho.0 == 0 {
        (koho.0, koho.1 / koho.0)
    } else {
        let q = koho.0;
        let p = (koho.1 as f64).sqrt() as usize;
        (p, q)
    };
    println!("{} {}", p, q);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

