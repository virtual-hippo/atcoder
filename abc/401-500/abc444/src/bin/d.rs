use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let m = 2 * 100_100;

    let mut counts = vec![0_usize; m];
    for &x in &a {
        counts[m - x] += 1;
    }

    // 累積和 — scanl1 (+) に対応
    let s = counts
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect_vec();

    // 繰り上がり — mapAccumR に対応（rev + scan + rev）
    let digits: Vec<_> = s
        .iter()
        .rev()
        .scan(0, |carry, &d| {
            let t = d + *carry;
            *carry = t / 10;
            Some(t % 10)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let ans: String = digits
        .iter()
        .skip_while(|&&d| d == 0)
        .map(|&d| char::from(d as u8 + b'0'))
        .collect();
    println!("{}", ans);
}
