use proconio::{fastout, input};

fn solve() {
    input! {
        n: usize,
        h: usize,
        tlu: [(usize,usize,usize); n],
    }

    let ok = (0..n)
        .fold(Some((0, h, h)), |acc, i| {
            if let Some((pt, pl, pu)) = acc {
                let (t, l, u) = tlu[i];
                let dt = t - pt;

                let nl = pl.saturating_sub(dt).max(l);
                let nu = (pu + dt).min(u);

                if u < nl || nu < l {
                    None
                } else {
                    Some((t, nl, nu))
                }
            } else {
                None
            }
        })
        .is_some();

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
