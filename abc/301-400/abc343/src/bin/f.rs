use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut current_a = a.clone();
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                current_a[p - 1] = x;
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = 0;
    println!("{}", ans);
}
