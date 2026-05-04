use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    if 3 <= x && x <= 6 + 6 + 6 {
        println!("Yes");
    } else {
        println!("No");
    }
}
