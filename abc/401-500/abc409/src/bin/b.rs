use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for x in (0..=n).rev() {
        let mut now = 0;
        for i in 0..n {
            if a[i] >= x {
                now += 1;
            }
        }

        if now >= x {
            println!("{}", x);
            break;
        }
    }
}
