use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }

    let mut ans = 0;

    for w in 0..=1_000_000 {
        let sign = if w % 2 == 0 { 1 } else { -1 };
        let nl = l.max(-w);
        let nr = r.min(w);
        let nd = d.max(-w);
        let nu = u.min(w);

        if nl <= nr && nd <= nu {
            ans += (nr - nl + 1) * (nu - nd + 1) * sign;
        }
    }
    println!("{}", ans);
}
