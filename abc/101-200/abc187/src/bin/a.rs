use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: String,
        b: String,
    }
    let a = a.chars().map(|ch| (ch as u8 - b'0') as u32).sum::<u32>();
    let b = b.chars().map(|ch| (ch as u8 - b'0') as u32).sum::<u32>();
    println!("{}", a.max(b));
}
