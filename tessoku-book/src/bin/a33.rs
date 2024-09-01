use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let val = a.iter().fold(0, |xor, v| v ^ xor);
    if val == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
