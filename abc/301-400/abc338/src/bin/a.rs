use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    for (i, ch) in s.chars().enumerate() {
        if i == 0 {
            if 97 <= (ch as u8) {
                println!("No");
                return;
            }
        } else {
            if (ch as u8) < 97 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
