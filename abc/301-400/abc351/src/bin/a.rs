use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    }
    let aa = a.iter().sum::<usize>();
    let bb = b.iter().sum::<usize>();
    let ans = aa - bb + 1;
    println!("{}", ans);
}
