use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }
    let s: usize = a.iter().sum();
    let mut current = t % s;
    for i in 0..n {
        if a[i] < current {
            current -= a[i];
        } else {
            println!("{} {}", i + 1, current);
            return;
        }
    }
}
