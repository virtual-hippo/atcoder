use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n*2],
    }

    let mut cnt = 0;

    for i in 0..2 * n - 1 {
        for j in (i + 1)..2 * n {
            if a[i] == a[j] {
                if i + 2 == j {
                    cnt += 1;
                }
                break;
            }
        }
    }

    let ans = cnt;
    println!("{}", ans);
}
