use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        apx: [(usize,usize,usize); n],
    }

    let ans = (0..n).fold(usize::MAX, |ans, i| {
        let (a, p, x) = apx[i];

        let x = x.saturating_sub(a);

        if x > 0 {
            ans.min(p)
        } else {
            ans
        }
    });

    if ans == usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
