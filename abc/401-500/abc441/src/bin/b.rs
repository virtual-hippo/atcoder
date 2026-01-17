use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: Chars,
        t: Chars,
        q: usize,
    }

    let s = s.iter().copied().collect::<HashSet<char>>();
    let t = t.iter().copied().collect::<HashSet<char>>();

    for _ in 0..q {
        input! {
            w: Chars,
        }
        let ok_takahashi = w.iter().all(|c| s.contains(c));
        let ok_aoki = w.iter().all(|c| t.contains(c));

        if ok_takahashi && !ok_aoki {
            println!("Takahashi");
        } else if !ok_takahashi && ok_aoki {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}
