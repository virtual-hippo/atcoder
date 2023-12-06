use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    if t.starts_with(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
