use proconio::input;

fn main() {
    input! {
        x: f64,
    }
    let x = (x * 10.0) as i32;
    if x % 10 > 4 {
        println!("{}", x / 10 + 1);
    } else {
        println!("{}", x / 10);
    }
}

