use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q],
    }

    let a = (0..q)
        .fold(((0..n).collect_vec(), (0..n).collect_vec()), |(mut a, mut b), i| {
            let bi = x[i];
            let ai = b[bi];
            let nai = if ai + 1 == n { ai - 1 } else { ai + 1 };
            let nbi = a[nai];

            (a[ai], a[nai]) = (a[nai], a[ai]);
            (b[bi], b[nbi]) = (b[nbi], b[bi]);

            (a, b)
        })
        .0
        .iter()
        .copied()
        .map(|x| x + 1)
        .collect_vec();

    print_vec_1line(&a);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
