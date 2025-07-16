use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }

    let mut masu = vec![false; n + 2];

    let mut black = 0;
    for i in 0..q {
        masu[a[i]] = !masu[a[i]];

        let l = masu[a[i] - 1];
        let r = masu[a[i] + 1];

        // 白 黒 白
        if !l && !r && masu[a[i]] {
            black += 1;
            println!("{}", black);
            continue;
        }

        // 黒 黒 黒
        if l && r && masu[a[i]] {
            black -= 1;

            println!("{}", black);
            continue;
        }

        // 黒 黒 白
        if l && !r && masu[a[i]] {
            println!("{}", black);
            continue;
        }

        // 白 黒 黒
        if !l && r && masu[a[i]] {
            println!("{}", black);
            continue;
        }

        // 白 白 白
        if !l && !r && !masu[a[i]] {
            black -= 1;
            println!("{}", black);
            continue;
        }

        // 黒 白 黒
        if l && r && !masu[a[i]] {
            black += 1;
            println!("{}", black);
            continue;
        }

        // 黒 白 白
        if l && !r && !masu[a[i]] {
            println!("{}", black);
            continue;
        }

        // 白 白 黒
        if !l && r && !masu[a[i]] {
            println!("{}", black);
            continue;
        }
    }
}
