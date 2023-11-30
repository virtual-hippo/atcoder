use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f64,
    }
    println!("{:?}", n.round() as i64);
}
