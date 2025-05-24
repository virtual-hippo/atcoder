use itertools::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    if n % 2 == 1 {
        println!("No");
        return;
    }

    let rle: Vec<(usize, &char)> = s.iter().dedup_with_count().collect_vec();

    let is_ok = rle.iter().all(|&(c, _)| c == 2) && rle.iter().map(|&(_, &ch)| ch).dedup_with_count().all(|(c, _)| c == 1);
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
