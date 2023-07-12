use proconio::input;

fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32,
    }
    let date = -a + b;
    if date <= 0 {
        println!("delicious");
    } else if x + 1 <= date {
        println!("dangerous");
    } else {
        println!("safe");
    }
}

