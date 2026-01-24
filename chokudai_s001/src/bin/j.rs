use ac_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mx = 100_100_usize;

    let (ans, _) = a
        .iter()
        .copied()
        .fold((0_usize, FenwickTree::new(mx, 0)), |(ans, mut tree), x| {
            tree.add(x, 1);
            (ans + (tree.sum(..mx) - tree.sum(..x + 1)), tree)
        });

    println!("{}", ans);
}
