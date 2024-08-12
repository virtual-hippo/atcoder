use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }
    if t > a {
        if n - (t + a) + a > t {
            println!("No");
        } else {
            println!("Yes");
        }
        return;
    } else if t < a {
        if n - (t + a) + t > a {
            println!("No");
        } else {
            println!("Yes");
        }
        return;
    }
    println!("No");
}
