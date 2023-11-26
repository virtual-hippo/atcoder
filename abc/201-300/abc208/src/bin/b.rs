use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut p: usize,
    }
    let mut koka = vec![1];
    for i in 2..12 {
        koka.push(koka[koka.len() - 1] * i);
    }
    let ans = (0..10).fold(0, |sum, i| {
        let residue = p % koka[i + 1];
        p -= residue;
        sum + (residue / koka[i])
    });
    println!("{}", ans);

    // let mut ans = 0;
    // let mut tail = koka.len() - 1;
    // while tail > 0 {
    //     let maisu = p / koka[tail];
    //     if maisu > 0 {
    //         p -= koka[tail] * maisu;
    //         ans += maisu;
    //     }
    //     tail -= 1;
    // }
    // println!("{}", ans + p);
}
