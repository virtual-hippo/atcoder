use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a <= b {
        println!("{}", b-a+1);
    } else {
        println!("0");
    }
}

