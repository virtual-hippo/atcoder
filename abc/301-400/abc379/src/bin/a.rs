use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    println!("{}{}{} {}{}{}", n[1], n[2], n[0], n[2], n[0], n[1]);
}
