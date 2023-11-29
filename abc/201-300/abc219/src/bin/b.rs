use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
        t: String,
    }
    for i in t.chars().map(|ch| (ch as usize - 48) - 1) {
        print!("{}", s[i]);
    }
}
