use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    if n == 1 {
        println!("0");
        return;
    }
    let mut num = n - 1;
    let mut result = String::new();
    while num != 0 {
        let remainder = num % 5;
        result.push_str(&(remainder * 2).to_string());
        num /= 5;
    }
    let ans = result.chars().rev().collect::<String>();
    println!("{}", ans);
}
