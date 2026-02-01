use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let ans = (n..)
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .take_while(|&sum| sum < k)
        .count();

    println!("{}", ans);
}
