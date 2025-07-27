use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _n: usize,
        l: usize,
        r: usize,
        s: Chars
    }

    let v = s
        .into_iter()
        .enumerate()
        .filter(|i| l <= i.0 + 1 && i.0 + 1 <= r)
        .all(|(_, v)| v == 'o');

    if v {
        println!("Yes");
    } else {
        println!("No");
    }
}
