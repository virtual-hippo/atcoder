use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.insert(k, x);

    for i in 0..n + 1 {
        print!("{} ", a[i]);
    }
}
