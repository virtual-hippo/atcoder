use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ie = vec![(0, 0); n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: char,
        }
        if b == 'M' {
            if ie[a].0 == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
            ie[a].0 += 1;
        } else {
            println!("No");
            ie[a].1 += 1;
        }
    }
}
