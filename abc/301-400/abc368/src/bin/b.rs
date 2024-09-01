use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut i = 0;
    loop {
        i += 1;
        a.sort_by(|a, b| b.cmp(a));
        a[0] -= 1;
        a[1] -= 1;
        if a.iter().filter(|&v| *v > 0).count() <= 1 {
            break;
        }
    }
    let ans = i;
    println!("{}", ans);
}
