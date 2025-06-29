use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut s = vec![];
    let mut c = vec![];

    for _ in 0..n {
        input! {
            si: String,
            ci: usize,
        }
        s.push(si);
        c.push(ci);
    }

    let sum = c.iter().sum::<usize>();

    let t = s
        .iter()
        .enumerate()
        .sorted_by_key(|(_, si)| si.clone())
        .map(|(i, _)| i)
        .collect_vec();

    let idx: usize = t[sum % n];

    println!("{}", s[idx]);
}
