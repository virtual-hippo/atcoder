use itertools::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
    }

    let p = p.iter().copied().chain([0]).filter(|&x| x <= m).sorted().collect_vec();

    let q = iproduct!(0..p.len(), 0..p.len())
        .map(|(i, j)| p[i] + p[j])
        .filter(|&x| x <= m)
        .unique()
        .sorted()
        .collect_vec();

    let ans = (0..q.len()).fold(0, |acc, i| {
        let j = q.partition_point(|&v| q[i] + v <= m).saturating_sub(1);
        acc.max(q[i] + q[j])
    });
    println!("{}", ans);
}
