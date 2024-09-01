use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    for i in 0..n {
        print!("{} ", a[(i + (n - k)) % n])
    }
}
