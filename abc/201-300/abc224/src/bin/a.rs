use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    if s.ends_with("er") {
        println!("er");
    } else {
        println!("ist");
    }
}
