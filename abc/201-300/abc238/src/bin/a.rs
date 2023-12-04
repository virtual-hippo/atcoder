use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    if n == 1 {
        println!("Yes");
    } else if n > 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
