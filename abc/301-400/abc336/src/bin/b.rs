use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let n = format!("{:b}", n);
    let mut ans = 0;
    for ch in n.chars().rev() {
        if ch == '0' {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
