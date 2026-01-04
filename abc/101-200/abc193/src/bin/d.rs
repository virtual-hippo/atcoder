use itertools::*;
use proconio::{fastout, input};

fn score(xs: &[usize]) -> usize {
    (1..10).map(|i| i * 10_usize.pow(xs[i] as u32)).sum::<usize>()
}

fn string_to_bucket(s: &String) -> [usize; 10] {
    s.chars().filter(|&c| c != '#').fold([0_usize; 10], |mut bucket, ch| {
        let x = ch as u8 - b'0';
        bucket[x as usize] += 1;
        bucket
    })
}

#[fastout]
fn main() {
    input! {
        k: usize,
        s: String,
        t: String,
    }

    let s = string_to_bucket(&s);
    let t = string_to_bucket(&t);

    let nokori = (1..10).fold([k; 10], |mut acc, i| {
        acc[i] -= s[i] + t[i];
        acc
    });

    let ans = iproduct!(1..10, 1..10)
        .filter(|&(i, j)| {
            let mut s = s.clone();
            let mut t = t.clone();
            s[i] += 1;
            t[j] += 1;

            score(&s) > score(&t)
        })
        .map(|(i, j)| {
            if i != j {
                (nokori[i] * nokori[j]) as f64 / ((9 * k - 8) * (9 * k - 9)) as f64
            } else {
                (nokori[i] * nokori[j].saturating_sub(1)) as f64 / ((9 * k - 8) * (9 * k - 9)) as f64
            }
        })
        .sum::<f64>();

    println!("{}", ans);
}
