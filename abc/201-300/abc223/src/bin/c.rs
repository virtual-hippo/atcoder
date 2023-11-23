use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64);n],
    }

    let t = ab.iter().map(|(a, b)| a / b).fold(0.0, |sum, v| sum + v) / 2.0;
    let mut current = 0.0;
    let mut ans = 0.0;
    for i in 0..n {
        if current + ab[i].0 / ab[i].1 < t {
            current += ab[i].0 / ab[i].1;
            ans += ab[i].0;
        } else {
            ans += ab[i].1 * (t - current);
            break;
        }
    }
    println!("{}", ans);
}
