use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    for i in 0..n {
        a[i] -= 1;
    }
    let mut is = vec![vec![]; n];
    for i in 0..n {
        is[a[i]].push(i + 1);
    }

    let c2 = |n: usize| (n * (n - 1)) / 2;
    let mut ans = 0;
    for x in 0..n {
        let mut sx = 0;
        {
            is[x].push(n + 1);
            let mut pre = 0;
            for i in is[x].iter() {
                sx += c2(i - pre);
                pre = *i;
            }
        }
        ans += c2(n + 1) - sx;
    }
    println!("{}", ans);
}
