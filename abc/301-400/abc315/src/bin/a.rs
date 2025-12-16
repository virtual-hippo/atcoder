use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let s = s
        .chars()
        .filter(|&c| c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u')
        .collect::<String>();

    println!("{}", s);
}
