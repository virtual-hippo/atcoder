use proconio::input;

fn f(vec: &Vec<usize>, k: usize) -> usize {
    let n = vec.len();
    if (n-1) % (k-1) == 0 {
        (n-1) / (k-1)
    } else {
        (n-1) / (k-1) + 1
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let ans = f(&a, k);
    println!("{}", ans);
}

