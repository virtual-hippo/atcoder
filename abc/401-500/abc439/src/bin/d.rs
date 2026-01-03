use itertools::*;
use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    if n < 3 {
        println!("{}", 0);
        return;
    }

    let a = a.iter().copied().map(|x| x * 5).collect_vec();

    let f = |indices: &[usize]| -> u64 {
        let mut res = 0;
        let mut map = HashMap::new();
        for &j in indices.iter() {
            *map.entry(a[j]).or_insert(0) += 1;

            let i = *map.get(&(a[j] / 5 * 7)).unwrap_or(&0);
            let k = *map.get(&(a[j] / 5 * 3)).unwrap_or(&0);
            res += i * k;
        }
        res
    };

    let ans = f(&(0..n).collect_vec()) + f(&(0..n).rev().collect_vec());
    println!("{}", ans);
}
