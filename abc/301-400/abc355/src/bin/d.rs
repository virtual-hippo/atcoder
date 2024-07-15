use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut l_list = vec![];
    let mut r_list = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        l_list.push(l);
        r_list.push(r);
    }
    l_list.sort();
    r_list.sort();

    let mut cnt = 0;
    let mut j = 0;
    for i in 0..n {
        while r_list[j] < l_list[i] {
            j += 1;
        }
        cnt += j;
    }
    let ans = (n * (n - 1) / 2) - cnt;
    println!("{}", ans);
}
