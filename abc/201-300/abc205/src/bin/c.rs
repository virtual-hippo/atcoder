use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: u32,
    }

    if c % 2 == 1 {
        if a < 0 && b < 0 {
            if a < b {
                println!("<");
            } else if a > b {
                println!(">");
            } else {
                println!("=");
            }
        } else if a < 0 {
            println!("<");
        } else if b < 0 {
            println!(">");
        } else {
            if a < b {
                println!("<");
            } else if a > b {
                println!(">");
            } else {
                println!("=");
            }
        }
    } else {
        if a.max(-a) < b.max(-b) {
            println!("<");
        } else if a.max(-a) > b.max(-b) {
            println!(">");
        } else {
            println!("=");
        }
    }
}
