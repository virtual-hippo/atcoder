use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64,
    }
    let sum = a.iter().sum::<u64>();

    let amari =  x % sum;
    let mut i = 0;
    let mut current = 0;
    while current <= amari {
        current += a[i];
        i += 1;
    }

    println!("{}", n as u64 * (x / sum) + i as u64);
}

