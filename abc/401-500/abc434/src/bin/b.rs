use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,f64); n],
    }
    let counter = ab.iter().fold(vec![(0_usize, 0.0); m], |mut acc, &(a, b)| {
        acc[a].0 += 1;
        acc[a].1 += b;
        acc
    });

    (0..m)
        .map(|k| counter[k].1 / (counter[k].0 as f64))
        .for_each(|avg| println!("{}", avg));
}
