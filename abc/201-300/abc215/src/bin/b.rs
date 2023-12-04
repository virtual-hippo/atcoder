use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut k = 0;
    while 2_u64.pow(k + 1) <= n {
        k += 1;
    }
    println!("{}", k);
}
