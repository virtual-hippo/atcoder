use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n-1],
    }
    let mut vec = vec![-1; n+1];

    for _ in 0..m {
        input! {
            x: usize,
            y: i64,
        }
        vec[x] = std::cmp::max(vec[x], y);
    }
    for i in 2..n+1 {
        vec[i] = std::cmp::max(vec[i], vec[p[i-2]]-1);
    }
    let ans = (1..n+1).filter(|&i| vec[i] >= 0).count();
    println!("{}", ans);
}
