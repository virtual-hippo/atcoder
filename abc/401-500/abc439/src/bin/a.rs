use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let ans = (1 << n) - (n << 1);
    println!("{}", ans);
}
