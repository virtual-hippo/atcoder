use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    a.sort_by(|x1, x2| x2.cmp(x1));
    b.sort_by(|x1, x2| x2.cmp(x1));

    let mut xx = 0;
    let mut yy = 0;
    for i in 0..n {
        xx += a[i];
        yy += b[i];
        if xx > x {
            println!("{}", i + 1);
            return;
        }
        if yy > y {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", n);
}
