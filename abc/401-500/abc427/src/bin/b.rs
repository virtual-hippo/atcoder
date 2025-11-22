use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![usize::MAX; n + 1];

    a[0] = 1;
    for i in 1..n + 1 {
        let mut now = 0;
        for j in 0..i {
            now += f(a[j]);
        }
        a[i] = now;
    }

    println!("{}", a[n]);
}

fn f(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    ret
}
