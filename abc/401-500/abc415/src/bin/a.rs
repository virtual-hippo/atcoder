use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    let ans = a.iter().find(|&&v| v == x).is_some();
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
