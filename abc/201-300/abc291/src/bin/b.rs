use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5 * n],
    }
    x.sort();
    let sum = (n..4*n).fold(0, |sum , i| sum + x[i]);
    println!("{}", sum as f64 / (3 * n) as f64);
}

