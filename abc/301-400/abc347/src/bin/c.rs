use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        d: [usize; n],
    }
    let c = a + b;
    let is_ok = d
        .into_iter()
        .map(|x| x % c)
        .unique()
        .flat_map(|x| [x, x + c])
        .sorted()
        .tuple_windows()
        .any(|(x, y)| y - x > b);

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
