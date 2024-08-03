use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cnt = 0;
    for _ in 0..n {
        input! {
            s: String,
        }
        if cnt == 2 {
            println!("No");
            return;
        }
        if s == "sweet" {
            cnt += 1;
        } else {
            cnt = 0;
        }
    }
    println!("Yes");
}
