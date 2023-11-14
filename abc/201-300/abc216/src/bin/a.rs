use proconio::input;

fn main() {
    input! {
        val: f64,
    }
    let x = val as i32;
    let y = (val * 10.0) as i32 % 10;
    if y <= 2 {
        println!("{}-", x);
    } else if y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
