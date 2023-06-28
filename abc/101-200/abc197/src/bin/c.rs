use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res = std::usize::MAX;
    for bit in 0..(1<<n-1) {
        let mut ored = 0;
        let mut xored = 0;
        for i in 0..n {
            ored |= a[i];
            if bit & (1 << i) == 0 {
                xored ^= ored;
                ored = 0;
            }
        }
        res = std::cmp::min(res, xored);
    }
    println!("{}", res);
}

