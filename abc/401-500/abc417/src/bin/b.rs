use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        b: [i32; m],
    }

    for i in 0..m {
        if let Some(j) = a.iter().position(|&x| x == b[i]) {
            a.remove(j);
        }
    }

    for v in a.iter() {
        print!("{} ", v);
    }
}
