use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }
    let mut aoki_hyo = ab.iter().fold(0, |sum, (v, _)| sum + *v);
    let mut takahashi_hyo = 0;
    ab.sort_by(|(a1, b1), (a2, b2)| (2 * a2 + b2).cmp(&(2 * a1 + b1)));

    let mut ans = 0;
    while takahashi_hyo <= aoki_hyo {
        takahashi_hyo += ab[ans].0 + ab[ans].1;
        aoki_hyo -= ab[ans].0;
        ans += 1;
    }
    println!("{}", ans);
}
