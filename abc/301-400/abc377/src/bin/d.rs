use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut min_list = vec![m; m];

    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        let l = l - 1;
        let r = r - 1;

        min_list[l] = min_list[l].min(r);
    }

    for l in (1..m).rev() {
        min_list[l - 1] = min_list[l - 1].min(min_list[l]);
    }

    let mut ans = 0;
    for l in 0..m {
        ans += min_list[l] - l;
    }

    println!("{}", ans);
}
