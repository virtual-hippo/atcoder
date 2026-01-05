use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        ca: [(usize, [usize; m]); n],
    }

    let ans = (0..(1 << n))
        .filter_map(|bit| {
            let (skill, tc) = (0..n)
                .filter(|&i| bit & (1 << i) != 0)
                .fold((vec![0; m], 0), |(mut skill, tc), i| {
                    let (c, a) = &ca[i];

                    (0..m).for_each(|j| skill[j] += a[j]);

                    (skill, tc + c)
                });

            if skill.iter().all(|&v| v >= x) {
                Some(tc)
            } else {
                None
            }
        })
        .min();

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
