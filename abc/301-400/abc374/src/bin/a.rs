use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    if s.ends_with("san") {
        println!("Yes");
    } else {
        println!("No");
    }
}
