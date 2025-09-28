use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut gohan = false;
    for c in s.chars() {
        if c == 'R' {
            gohan = true;
        }

        if c == 'M' && gohan {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
