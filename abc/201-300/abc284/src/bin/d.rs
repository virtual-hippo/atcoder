use proconio::{fastout, input};

fn solve() {
    input! {
        n: usize,
    }

    let is_p = |x: usize| n % x.pow(2) == 0;

    let (p, q) = (2_usize..)
        .take_while(|&x| x.pow(3) <= n)
        .find(|&x| n % x == 0)
        .map(|x| if is_p(x) { (x, n / x.pow(2)) } else { ((n / x).isqrt(), x) })
        .unwrap();

    println!("{} {}", p, q);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
