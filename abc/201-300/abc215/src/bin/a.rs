use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    if s == "Hello,World!".to_string() {
        println!("AC");
    } else {
        println!("WA");
    }
}
