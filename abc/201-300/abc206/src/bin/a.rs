use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n * 108 / 100 < 206 {
        println!("Yay!");
    } else if n * 108 / 100 == 206 {
        println!("so-so");
    } else {
        println!(":(");
    }
}
