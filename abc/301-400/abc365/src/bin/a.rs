use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        y: usize,
    }
    if y % 4 != 0 {
        println!("365");
    } else if y % 100 != 0 {
        println!("366");
    } else if y % 100 == 0 && y % 400 != 0 {
        println!("365");
    } else {
        println!("366");
    }
}
