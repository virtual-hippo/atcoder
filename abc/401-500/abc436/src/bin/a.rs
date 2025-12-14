use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    let ch = (0..n - s.len()).map(|_| 'o').collect::<String>();
    println!("{}{}", ch, s);
}
