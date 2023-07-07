use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ans = (0,0);
    for i in 0..n {
        if a[i] == b[i] {
            ans.0 += 1;
        }
        for j in 0..n {
            if a[i] == b[j] && i != j {
                ans.1 += 1;
            }
        }
    }
    println!("{}\n{}", ans.0, ans.1);
}

