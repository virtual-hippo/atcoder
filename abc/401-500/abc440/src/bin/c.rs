use proconio::{fastout, input};

fn solve() {
    input! {
        n: usize,
        w: usize,
        c: [usize; n],
    }
    let xs = (0..n)
        .map(|i| (1 + i) % (2 * w))
        .enumerate()
        .fold(vec![0; 4 * w], |mut xs, (i, v)| {
            xs[v] += c[i];
            xs[v + 2 * w] += c[i];
            xs
        });

    let init = (0..w).map(|i| xs[i]).sum::<usize>();
    let ans = (w..xs.len())
        .fold((init, init), |(ans, old), i| {
            let new = old + xs[i] - xs[i - w];
            (new.min(ans), new)
        })
        .0;

    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
