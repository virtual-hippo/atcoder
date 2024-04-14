use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u64,
        r: u64,
    }
    let mut ll = l;
    let mut ans = vec![];
    while ll < r {
        let mut i = 0;
        while ll % 2_u64.pow(i + 1) == 0 && 2_u64.pow(i + 1) + ll <= r {
            i += 1;
        }
        ans.push((ll, ll + 2_u64.pow(i)));
        ll += 2_u64.pow(i);
    }
    println!("{}", ans.len());
    for pair in ans.iter() {
        println!("{} {}", pair.0, pair.1);
    }
}
