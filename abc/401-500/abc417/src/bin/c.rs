use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counter = vec![0; n];

    let mut ans: u64 = 0;

    for i in 0..n {
        if i + a[i] < n {
            counter[i + a[i]] += 1;
        }

        if i < a[i] {
            continue;
        }

        ans += counter[i - a[i]];
    }

    println!("{}", ans);
}
