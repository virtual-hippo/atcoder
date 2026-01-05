use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let ans = (1_usize..)
        .take_while(|&a| a <= (m.isqrt() + 1))
        .filter_map(|a| {
            let b = if m / a * a >= m { m / a } else { m / a + 1 };
            if a <= n && b <= n {
                Some(a * b)
            } else {
                None
            }
        })
        .min();

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
