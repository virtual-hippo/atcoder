use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    for i in 0..n {
        let s: usize = (0..n).filter(|&j| j != i).map(|j| a[j]).sum();
        if s == m {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
