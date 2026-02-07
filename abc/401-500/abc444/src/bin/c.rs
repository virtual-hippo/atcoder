use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a = a.iter().copied().sorted().collect_vec();
    if n % 2 == 1 {
        println!("{}", a[n - 1]);
        return;
    }

    let mn = a[0];
    let mx = a[n - 1];

    let mut candidate = vec![];
    if (0..n / 2).all(|i| a[i] + a[n - 1 - i] == mn + mx) {
        candidate.push(mn + mx);
    }

    {
        let a = a.iter().copied().filter(|&x| x != mx).collect_vec();
        let n = a.len();
        if n % 2 == 0 && (0..n / 2).all(|i| a[i] + a[n - 1 - i] == mx) {
            candidate.push(mx);
        }
    }

    let ans = candidate.iter().copied().unique().sorted().collect_vec();

    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
