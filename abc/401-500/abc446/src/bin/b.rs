use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut is_used = vec![false; m];
    for _ in 0..n {
        input! {
            l: usize,
            x: [Usize1; l],
        }

        if let Some(&j) = x.iter().find(|&&i| !is_used[i]) {
            is_used[j] = true;
            println!("{}", j + 1);
        } else {
            println!("{}", 0);
        }
    }
}
