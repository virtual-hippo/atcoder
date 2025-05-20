use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut a = vec![];

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                }
                a.push(x);
            },
            2 => {
                input! {
                    k: usize,
                }
                println!("{}", a[a.len() - k]);
            },
            _ => unreachable!(),
        }
    }
}
