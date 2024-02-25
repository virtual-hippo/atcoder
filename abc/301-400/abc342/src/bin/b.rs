use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
    }
    let mut order = vec![0; n];
    for i in 0..n {
        order[p[i] - 1] = i;
    }
    for _ in 0..q {
        input! {
            (a,b): (usize,usize),
        }
        if order[a - 1] > order[b - 1] {
            println!("{}", b);
        } else {
            println!("{}", a);
        }
    }
}
