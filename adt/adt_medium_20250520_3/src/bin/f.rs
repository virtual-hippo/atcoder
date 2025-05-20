use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }
    let mut row = vec![0; n];
    let mut col = vec![0; n];

    let mut n0 = 0;
    let mut n1 = 0;

    for ai in 0..t {
        let v = a[ai] - 1;
        let r = v / n;
        let c = v % n;

        row[r] += 1;
        col[c] += 1;

        if r == c {
            n0 += 1;
        }
        if r + c == n - 1 {
            n1 += 1;
        }

        if row[r] == n || col[c] == n || n0 == n || n1 == n {
            println!("{}", ai + 1);
            return;
        }
    }

    let ans = -1;
    println!("{}", ans);
}
