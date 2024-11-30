use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..i + 1 {
            input! {
                v: usize,
            }
            a[i].push(v)
        }
    }
    let mut i = 0;
    for j in 0..n {
        if i >= j {
            i = a[i][j] - 1;
        } else {
            i = a[j][i] - 1;
        }
    }
    let ans = i;
    println!("{}", ans + 1);
}
