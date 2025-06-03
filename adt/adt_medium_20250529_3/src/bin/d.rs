use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: Usize1,
        c: [Usize1; n],
        r: [Usize1; n],
    }

    let exist_t = c.iter().any(|&x| x == t);

    let t = if exist_t { t } else { c[0] };

    let ans = (0..n).filter(|&i| c[i] == t).max_by_key(|&i| r[i]).unwrap() + 1;

    println!("{}", ans);
}
