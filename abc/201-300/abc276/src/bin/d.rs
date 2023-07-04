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
        n: usize,
        a: [usize; n],
    }

    let g = a.iter().fold(0, |g, &x| gcd(g,x));
    let mut ans = 0;
    for i in 0..n {
        let mut current = a[i];
        current /= g;
        while current % 2 == 0 {
            current /= 2;
            ans += 1;
        }
        while current % 3 == 0 {
            current /= 3;
            ans += 1;
        }
        if current != 1 {
            println!("{}", -1);
            return;
        }
    }
    
    println!("{}", ans);
}

