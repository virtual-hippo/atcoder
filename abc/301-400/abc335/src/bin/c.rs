use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        q: usize,
    }
    // let mut state = vec![(0, 0); n + 1];
    // for i in 1..=n {
    //     state[i].0 = i;
    // }
    let mut cnt = 0;
    let mut ido = vec![(1, 0)];
    for _ in 0..q {
        input! {
            x: usize,
        }
        if x == 1 {
            input! {
                c: char,
            }
            cnt += 1;
            if c == 'U' {
                ido.push((ido[ido.len() - 1].0, ido[ido.len() - 1].1 + 1));
            } else if c == 'D' {
                ido.push((ido[ido.len() - 1].0, ido[ido.len() - 1].1 - 1));
            } else if c == 'R' {
                ido.push((ido[ido.len() - 1].0 + 1, ido[ido.len() - 1].1));
            } else if c == 'L' {
                ido.push((ido[ido.len() - 1].0 - 1, ido[ido.len() - 1].1));
            }
        } else {
            input! {
                p: i32,
            }
            if cnt <= p - 1 {
                println!("{} {}", p - cnt, 0);
            } else {
                let ind = (cnt - (p - 1)) as usize;
                let ans = (ido[ind].0, ido[ind].1);
                println!("{} {}", ans.0, ans.1);
            }
        }
    }
}
