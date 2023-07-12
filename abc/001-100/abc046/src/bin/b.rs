use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let ans = (0..n).fold(1, |ans, i| if i == 0 { ans * k } else { ans * (k - 1) });
    println!("{}", ans);
}
