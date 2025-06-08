use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n - 1],
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let d = std::iter::once(0_usize)
        .chain(d.iter().scan(0_usize, |acc, &x| {
            *acc += x;
            Some((*acc) % l)
        }))
        .collect::<Vec<_>>();

    let mut counter = vec![0; l];
    for i in 0..n {
        counter[d[i]] += 1;
    }

    let ans = (0..(l / 3))
        .map(|i| counter[i] * counter[i + l / 3] * counter[i + (2 * l) / 3])
        .sum::<u64>();
    println!("{}", ans);
}
