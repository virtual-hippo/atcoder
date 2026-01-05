use itertools::*;
use proconio::{fastout, input};
use std::collections::*;

pub fn get_primes(n: usize) -> Vec<usize> {
    (1_usize..)
        .take_while(|&x| x.pow(2) <= n)
        .filter(|&x| n % x == 0)
        .map(|x| if x * x == n { vec![x] } else { vec![x, n / x] })
        .flatten()
        .sorted()
        .collect_vec()
}

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let primes = get_primes(k);

    let map = primes.iter().copied().fold(HashMap::new(), |mut map, prime| {
        map.insert(prime, get_primes(prime));
        map
    });

    let ans = primes
        .iter()
        .copied()
        .map(|x| {
            let y = k / x;
            map[&x].iter().copied().filter(move |&a| a <= x / a).map(move |a| {
                let b = x / a;

                let mut abc = [a, b, y];
                abc.sort();
                abc
            })
        })
        .flatten()
        .unique()
        .count();
    println!("{}", ans);
}
