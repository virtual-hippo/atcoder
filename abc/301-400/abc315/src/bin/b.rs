use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }

    let mid = d.iter().copied().sum::<usize>() / 2 + 1;

    let mut i = 0;
    for a in 0..m {
        for b in 0..d[a] {
            i += 1;
            if i == mid {
                println!("{} {}", a + 1, b + 1);
            }
        }
    }
}
