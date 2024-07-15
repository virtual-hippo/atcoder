use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize,usize); n],
    }
    let sum = pq.into_iter().fold(0, |sum, (p, q)| sum + p * q);
    let ans = if sum >= s { sum } else { sum + k };
    println!("{}", ans);
}
