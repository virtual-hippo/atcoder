use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![]; n];

    for i in 0..n {
        for j in 0..i + 1 {
            if j == 0 || j == i {
                a[i].push(1)
            } else {
                let v = a[i - 1][j - 1] + a[i - 1][j];
                a[i].push(v);
            }
        }
    }

    for i in 0..n {
        for j in 0..i + 1 {
            print!("{} ", a[i][j]);
        }
        println!()
    }
}
