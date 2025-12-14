use ac_library::*;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut dsu = Dsu::new(n);

    for i in 0..n {
        dsu.merge(i, p[i]);
    }

    let parents = (0..n).map(|i| dsu.leader(i)).unique().collect_vec();

    let ans = parents
        .iter()
        .copied()
        .map(|i| dsu.size(i))
        .map(|x| (x * (x - 1)) / 2)
        .sum::<usize>();
    println!("{}", ans);
}
