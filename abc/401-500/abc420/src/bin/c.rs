use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    let mut s = (0..n).map(|i| a[i].min(b[i])).sum::<i64>();

    for _ in 0..q {
        input! {
            c: char,
        }

        match c {
            'A' => {
                input! {
                    x: usize,
                    v: i64,
                }
                let x = x - 1;

                let pre = a[x].min(b[x]);
                a[x] = v;
                let post = a[x].min(b[x]);
                s += post - pre;
                println!("{}", s);
            },
            'B' => {
                input! {
                    x: usize,
                    v: i64,
                }
                let x = x - 1;

                let pre = a[x].min(b[x]);
                b[x] = v;
                let post = a[x].min(b[x]);
                s += post - pre;
                println!("{}", s);
            },
            _ => unreachable!(),
        }
    }
}
