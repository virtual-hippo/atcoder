use itertools::*;
use proconio::{fastout, input};

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

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let sosu = sieve_of_eratosthenes(1_000_010)
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i)
        .collect_vec();

    let ans = sosu
        .iter()
        .copied()
        .take_while(|&q| q.pow(3) <= n)
        .map(|q| {
            sosu.iter()
                .copied()
                .take_while(|&p| q.pow(3) * p <= n)
                .filter(|&p| p < q && q.pow(3) * p <= n)
                .count() as u64
        })
        .sum::<u64>();
    println!("{}", ans);
}
