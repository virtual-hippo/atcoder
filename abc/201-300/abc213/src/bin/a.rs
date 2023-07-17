use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    for c in 0..256 {
        if a ^ c == b {
            println!("{}", c);
            return;
        }
    }
}

