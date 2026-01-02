use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
    }

    let cnt_1 = s.iter().filter(|&&c| c == '1').count() as i64;
    let mut c = vec![0_i64; n + 1];

    for i in 0..n {
        c[i + 1] = if s[i] == '0' { 1 } else { -1 } + c[i];
    }

    let mut res = 0;
    let mut mx = 0;

    for i in 0..n + 1 {
        res = res.min(c[i] - mx);
        mx = mx.max(c[i]);
    }
    let ans = res + cnt_1;
    println!("{}", ans);
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
    }

    let dp = (0..n).fold([0, 0, 0], |old: [usize; 3], i| {
        let (cost_0, cost_1) = if s[i] == '0' { (0, 1) } else { (1, 0) };
        [old[0] + cost_0, old[0].min(old[1]) + cost_1, old[1].min(old[2]) + cost_0]
    });

    let ans = dp.into_iter().min().unwrap();
    println!("{}", ans);
}
