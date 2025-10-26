use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for i in 0..n {
        let i = i + 1;
        if i <= m {
            println!("OK");
        } else {
            println!("Too Many Requests");
        }
    }
}
