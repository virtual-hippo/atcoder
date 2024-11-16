use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n-1],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort();

    let mut vec = vec![];

    for i in 0..n {
        if b.len() > 0 && a[i] <= b[b.len() - 1] {
            b.pop();
        } else {
            vec.push(a[i]);
        }
    }

    if vec.len() > 1 {
        println!("{}", -1);
    } else if vec.len() == 1 {
        println!("{}", vec[0]);
    }
}
