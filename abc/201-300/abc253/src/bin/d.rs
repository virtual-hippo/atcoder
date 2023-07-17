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

fn lcm(x: usize, y: usize) -> usize {
    let d = gcd(x,y);
    x / d * y
}

fn souwa(num: usize, coeficient: usize) -> usize {
    ((1+num) * num * coeficient) / 2
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let koubaisu = lcm(a, b);
    let ans = souwa(n, 1) - (souwa(n / a, a) + souwa(n / b, b) - souwa(n / koubaisu, koubaisu));
    println!("{}", ans);
}

