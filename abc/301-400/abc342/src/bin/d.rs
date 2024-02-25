use proconio::{fastout, input};
use std::collections::HashMap;

pub fn prime_factorize(input: u64) -> u64 {
    let mut target = input;
    let mut i = 2;
    let mut ret = 1;

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
        if e % 2 == 1 {
            ret *= i;
        }

        i += 1;
    }
    if target != 1 {
        ret *= target;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let b = a
        .iter()
        .map(|&val| if val == 0 { val } else { prime_factorize(val) })
        .collect::<Vec<u64>>();

    let mut map = HashMap::new();
    for &val in b.iter() {
        *map.entry(val).or_insert(0) += 1;
    }

    let mut ans: u64 = 0;
    let mut cnt_0 = 0;
    for i in 0..n {
        if a[i] == 0 {
            cnt_0 += 1;
            ans += (n - cnt_0) as u64;
        }
    }

    for (&k, &v) in map.iter() {
        if k == 0 {
            continue;
        }
        ans += (v * (v - 1)) / 2;
    }
    println!("{}", ans);
}
