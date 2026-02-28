use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n / 2 + n % 2 >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
