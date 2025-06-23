use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut ans = vec![];

    for ch in s.chars().rev() {
        if ch == '.' {
            break;
        }
        ans.push(ch);
    }
    let ans = ans.iter().rev().collect::<String>();
    println!("{}", ans);
}
