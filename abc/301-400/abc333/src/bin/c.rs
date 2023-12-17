use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut i = 1;
    let mut repunits = [1, 1, 1];
    while i < n {
        if repunits[0] == repunits[1] && repunits[1] == repunits[2] {
            repunits[0] = repunits[0] * 10 + 1;
            repunits[1] = 1;
            repunits[2] = 1;
        } else if repunits[1] == repunits[2] {
            repunits[1] = repunits[1] * 10 + 1;
            repunits[2] = 1;
        } else {
            repunits[2] = repunits[2] * 10 + 1;
        }
        i += 1;
    }
    let ans = repunits.iter().sum::<u64>();
    println!("{}", ans);
}
