use itertools::*;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: u64,
    }

    let ans = (1u64..)
        .take_while(|&x| {
            let y = x + 1;
            x * x + y * y <= n
        })
        .flat_map(|x| ((x + 1)..).take_while(move |&y| x * x + y * y <= n).map(move |y| x * x + y * y))
        .fold(FxHashMap::default(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        })
        .iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(&k, _)| k)
        .sorted()
        .collect_vec();

    println!("{}", ans.len());
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
