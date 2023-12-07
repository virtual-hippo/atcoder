use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }
    let val = s.chars().nth(n - 1).unwrap();
    if val == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
