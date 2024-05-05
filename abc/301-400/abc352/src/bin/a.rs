use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        x: usize,
        y: usize,
        z: usize,
    }
    if x < z && z < y {
        println!("Yes");
    } else if y < z && z < x {
        println!("Yes");
    } else {
        println!("No");
    }
}
