use proconio::{fastout, input};

fn solve() {
    input! {
        n: usize,
        r: [usize; n],
    }

    let (ans, _) = (0..n - 1)
        .map(|j| (j, j + 1))
        .chain((1..n).rev().map(|j| (j, j - 1)))
        .fold((0, r), |(ans, mut r), (from, to)| {
            let limit = r[from] + 1;
            let diff = r[to].saturating_sub(limit);
            if diff > 0 {
                r[to] = limit;
            }
            (ans + diff, r)
        });

    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(|_| solve());
}
