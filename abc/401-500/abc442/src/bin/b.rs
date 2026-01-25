use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        a: [usize; q],
    }

    let (_, _, ans) = a.iter().fold((0_usize, false, vec![]), |(v, f, mut ans), &a| {
        let (v, f) = match a {
            1 => (v + 1, f),
            2 => (v.saturating_sub(1), f),
            3 => (v, !f),
            _ => unreachable!(),
        };
        ans.push(v >= 3 && f);
        (v, f, ans)
    });

    for i in 0..q {
        if ans[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
