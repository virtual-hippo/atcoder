use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    }
    let mut it = (a - 1);
    for i in 1..k {
        it += 1;
        it %= n;
    }
    println!("{}", it + 1);
}
