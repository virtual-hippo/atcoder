use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in 0..n {
        let mut r = usize::MAX;
        for j in 0..i {
            if a[j] > a[i] {
                r = j;
            }
        }

        if r == usize::MAX {
            println!("-1");
        } else {
            println!("{}", r + 1);
        }
    }
}
