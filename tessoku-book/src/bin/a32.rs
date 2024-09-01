use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut dp = vec![false; n + a + b];
    for i in 0..=n {
        if i >= a && dp[i - a] == false {
            dp[i] = true;
        } else if i >= b && dp[i - b] == false {
            dp[i] = true;
        } else {
            dp[i] = false;
        }
    }
    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
