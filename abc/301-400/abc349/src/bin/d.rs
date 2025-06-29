use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u64,
        r: u64,
    }
    let mut ans: Vec<(u64, u64)> = vec![];

    let mut ll = l;
    while ll < r {
        let mut rr = 0;
        for i in 0..62 {
            let ii = 1 << i;
            if ll % ii != 0 {
                continue;
            }
            let nr = (ll / ii + 1) * ii;
            if nr > r {
                break;
            }
            rr = nr;
        }
        ans.push((ll, rr));
        ll = rr;
    }

    println!("{}", ans.len());
    for v in ans.iter() {
        println!("{} {}", v.0, v.1);
    }
}
