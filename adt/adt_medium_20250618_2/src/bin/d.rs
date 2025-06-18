use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ns = format!("{:b}", n);

    let mut ans = 0;
    for ch in ns.chars().rev() {
        if ch == '1' {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
