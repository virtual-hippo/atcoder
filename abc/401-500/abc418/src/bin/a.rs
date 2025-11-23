use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }

    if s.ends_with("tea") {
        println!("Yes");
    } else {
        println!("No");
    }
}
