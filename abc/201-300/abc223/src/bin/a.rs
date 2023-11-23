use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    if x == 0 {
        println!("No");
        return;
    }
    if x % 100 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
