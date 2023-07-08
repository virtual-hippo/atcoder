use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let ans1 = (0..n).filter(|&i| s[i] == 'o').count() > 0;
    let ans2 = (0..n).filter(|&i| s[i] == 'x').count() == 0;
    if ans1 && ans2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
