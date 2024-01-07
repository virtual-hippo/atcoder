use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![];
    for a in 0..=n {
        for b in 0..=n {
            for c in 0..=n {
                if a + b + c <= n {
                    ans.push((a, b, c));
                }
            }
        }
    }

    ans.sort();
    for &v in ans.iter() {
        println!("{} {} {}", v.0, v.1, v.2);
    }
}
