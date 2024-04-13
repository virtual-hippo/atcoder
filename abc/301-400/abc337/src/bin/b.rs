use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut current = 'A';
    for ch in s.chars() {
        if current == 'A' {
            if ch == 'A' {
                //
            } else if ch == 'B' {
                current = 'B';
            } else {
                current = 'C';
            }
        } else if current == 'B' {
            if ch == 'A' {
                println!("No");
                return;
            } else if ch == 'B' {
                continue;
            } else {
                current = 'C';
            }
        } else if current == 'C' {
            if ch == 'A' {
                println!("No");
                return;
            } else if ch == 'B' {
                println!("No");
                return;
            } else {
                //
            }
        }
    }
    println!("Yes");
}
