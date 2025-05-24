use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 12],
    }

    let ans = s.iter().filter(|&ss| ss.contains("r")).count();
    println!("{}", ans);
}
