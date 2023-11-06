use proconio::input;

fn main() {
    input! {
        b: u64,
    }
    let mut i: u64 = 1;
    while i.pow(i as u32) <= b {
        if i.pow(i as u32) == b {
            println!("{}", i);
            return;
        }
        i += 1;
    }
    println!("{}", -1);
}
