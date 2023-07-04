use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    let mut xy = (x, y);
    while xy.0 >= 1 && xy.1 >= 1 {
        if xy.0 < xy.1 {
            xy.1 %= xy.0;
        } else {
            xy.0 %= xy.1;
        }
    }
    if xy.0 >= 1 {
        xy.0
    } else {
        xy.1
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let g = gcd(a,b);

    let mut kouyakusu = Vec::with_capacity(100_000);
    {
        let mut i = 1;
        while i * i <= g {
            if g % i == 0 {
                kouyakusu.push(i);
                if i != g/i {
                    kouyakusu.push(g/i);
                }
            }
            i += 1;
        }
    }
    kouyakusu.sort();

    let mut ans = vec![1];
    for i in 1..kouyakusu.len() {
        let mut flag = true;
        for j in 0..ans.len() {
            if gcd(kouyakusu[i], ans[j]) != 1 {
                flag = false;
                break;
            }
        }
        if flag {
            ans.push(kouyakusu[i]);
        }
    }
    println!("{}", ans.len());
}

