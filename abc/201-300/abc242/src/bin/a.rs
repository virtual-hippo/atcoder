use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    if x <= a {
        print!("1.000000000000");
    } else if x <= b {
        print!("{}", c as f64 / (b as f64 - a as f64));
    } else {
        print!("0.000000000000");
    }
}

