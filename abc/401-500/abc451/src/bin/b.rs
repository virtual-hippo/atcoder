use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1);n],
    }

    let (x, y) = ab.iter().fold((vec![0; m], vec![0; m]), |(mut x, mut y), &(a, b)| {
        x[a] += 1;
        y[b] += 1;
        (x, y)
    });

    for i in 0..m {
        let ans = y[i] - x[i];
        println!("{}", ans);
    }
}
