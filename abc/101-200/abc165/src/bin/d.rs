use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    }
    let x = n.min(b-1);
    println!("{}", (a * x) / b - a * (x / b));
}
