use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for l in 1..=n {
        for r in l..=n {
            let s = (l..=r).map(|i| a[i - 1]).sum::<usize>();
            let ok = (l..=r).all(|i| {
                let i = i - 1;
                s % a[i] != 0
            });

            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
