use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }
    if (l, r) == (1, 0) {
        println!("Yes");
    } else if (l, r) == (0, 1) {
        println!("No");
    } else {
        println!("Invalid");
    }
}
