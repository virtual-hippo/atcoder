use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let (a, b, c) = (n[0], n[1], n[2]);
    print!("{}{}{} {}{}{}", b, c, a, c, a, b);
}
