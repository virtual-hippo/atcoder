use proconio::{fastout, input};
use std::collections::HashSet;

pub fn solve(n: usize, set: &mut HashSet<usize>) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = true;

    for &soinsu in set.iter() {
        if soinsu > n {
            continue;
        }
        if is_prime[soinsu] == false {
            continue;
        }
        let mut j = soinsu;
        while j < n + 1 {
            is_prime[j] = false;
            j += soinsu;
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|(_, flag)| **flag)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

pub fn prime_factorize(input: usize, set: &mut HashSet<usize>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i <= target {
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
        set.insert(i);

        i += 1;
    }
    if target != 1 {
        res.push((target, 1));
        set.insert(target);
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();
    for i in 0..n {
        prime_factorize(a[i], &mut set);
    }
    let ans = solve(m, &mut set);

    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}
