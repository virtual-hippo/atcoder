use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }
    let mut cnt = 0;
    let mut current = a;
    while current < b {
        current *= k;
        cnt += 1;
    }
    println!("{}", cnt);
}

