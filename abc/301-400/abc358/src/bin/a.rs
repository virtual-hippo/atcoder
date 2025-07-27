use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    if (s, t) == ("AtCoder".to_string(), "Land".to_string()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
