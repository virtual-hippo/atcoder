use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let set = a.into_iter().collect::<std::collections::HashSet<_>>();

    let ans = (1..=n)
        .filter(|&x| !set.contains(&x))
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    println!("{}", ans.len());
    println!("{}", ans.join(" "));
}
