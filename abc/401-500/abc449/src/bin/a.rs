use proconio::input;

fn main() {
    input! {
        d: usize,
    }

    let d = d as f64;
    let r = d / 2.0;

    let ans = r * r * std::f64::consts::PI;
    println!("{}", ans);
}
