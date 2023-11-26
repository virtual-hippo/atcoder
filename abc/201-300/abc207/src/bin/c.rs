use proconio::{fastout, input};

fn get_lr((t, l, r): (usize, i64, i64)) -> (i64, i64) {
    match t {
        1 => (2 * l, 2 * r + 1),
        2 => (2 * l, 2 * r),
        3 => (2 * l + 1, 2 * r + 1),
        4 => (2 * l + 1, 2 * r),
        _ => unreachable!(),
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        tlr: [(usize,i64,i64); n],
    }
    let ans = (0..n - 1)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .map(|(i, j)| ((get_lr(tlr[i])), get_lr(tlr[j])))
        .filter(|&((l1, r1), (l2, r2))| 1 <= r1.min(r2) - l1.max(l2))
        .count();
    println!("{}", ans);
}
