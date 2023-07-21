use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in (0..n+1).rev() {
        println!("{}", i);
    }
}

