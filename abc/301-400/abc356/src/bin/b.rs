use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut s = vec![0; m];
    for _ in 0..n {
        for i in 0..m {
            input! { x: usize }
            s[i] += x;
        }
    }

    let ok = (0..m).all(|i| s[i] >= a[i]);

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
