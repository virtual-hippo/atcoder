use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        mut ab: [(u64,u64); n],
    }
    let mut nokori = k;
    let mut current = 0;
    ab.sort();
    for i in 0..n {
        if ab[i].0 - current <= nokori {
            nokori -= ab[i].0 - current;
            current = ab[i].0;
            nokori += ab[i].1;
        } else {
            break;
        }
    }
    current += nokori;
    println!("{}", current);
}
