use proconio::{fastout, input};

pub fn digits_to_vec(x: usize) -> Vec<usize> {
    if x / 10 == 0 {
        vec![x % 10]
    } else {
        digits_to_vec(x / 10).into_iter().chain(std::iter::once(x % 10)).collect()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ans = (1..n + 1).map(|x| digits_to_vec(x).len()).filter(|&x| x & 1 == 1).count();
    println!("{}", ans);
}
