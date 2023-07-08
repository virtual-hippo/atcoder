use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut cnt = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            cnt[(n + p[i] - 1 + j - i) as usize % n] += 1;
        }
    }
    println!("{}", cnt.iter().max().unwrap());
}
