use itertools::*;
use proconio::input;

fn main() {
    input! {
        a: [[usize; 6]; 3],
    }
    let v = iproduct!(0..6, 0..6, 0..6)
        .filter(|&(i, j, k)| {
            let mut counter = [0; 7];
            counter[a[0][i]] += 1;
            counter[a[1][j]] += 1;
            counter[a[2][k]] += 1;
            counter[4] == 1 && counter[5] == 1 && counter[6] == 1
        })
        .count();

    let p = v as f64 / (6 * 6 * 6) as f64;

    println!("{}", p);
}
