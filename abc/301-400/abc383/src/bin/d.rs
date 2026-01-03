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

    let sosu = sieve_of_eratosthenes(2_000_000)
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i)
        .collect_vec();

    let ans0 = sosu
        .iter()
        .filter(|&&x| x <= (n.isqrt() + 1).isqrt().isqrt())
        .take_while(|x| x.pow(8 as u32) <= n)
        .count();

    let ans1 = (0..sosu.len())
        .filter(|&i| sosu[i] * sosu[i] <= n)
        .map(|i| {
            let pp = sosu[i] * sosu[i];

            ((i + 1)..sosu.len())
                .take_while(|&j| sosu[j] * sosu[j] <= n / pp + 1)
                .filter(|&j| pp * sosu[j] * sosu[j] <= n)
                .count()
        })
        .sum::<usize>();

    println!("{}", ans0 + ans1);
}
