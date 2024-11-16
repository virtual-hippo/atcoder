use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let (mut l, mut r) = (0, 1);
    let mut ans = 0;
    for _ in 0..q {
        input! {
            h: char,
            t: usize,
        }
        let t = t - 1;

        match h {
            'L' => {
                let v0 = {
                    let mut now = l;
                    let mut ret = 0;
                    while now != t {
                        now += 1;
                        now = (now + n) % n;
                        ret += 1;
                        if now == r {
                            break;
                        }
                    }
                    if now == t {
                        Some(ret)
                    } else {
                        None
                    }
                };
                let v1 = {
                    let mut now = l;
                    let mut ret = 0;
                    while now != t {
                        now += n;
                        now -= 1;
                        now = (now + n) % n;
                        ret += 1;
                        if now == r {
                            break;
                        }
                    }
                    if now == t {
                        Some(ret)
                    } else {
                        None
                    }
                };
                l = t;
                if let (Some(val0), Some(val1)) = (v0, v1) {
                    ans += val0.min(val1);
                } else if let Some(val) = v0 {
                    ans += val;
                } else if let Some(val) = v1 {
                    ans += val;
                }
            }
            'R' => {
                let v0 = {
                    let mut now = r;
                    let mut ret = 0;
                    while now != t {
                        now += 1;
                        now = (now + n) % n;
                        ret += 1;
                        if now == l {
                            break;
                        }
                    }
                    if now == t {
                        Some(ret)
                    } else {
                        None
                    }
                };
                let v1 = {
                    let mut now = r;
                    let mut ret = 0;
                    while now != t {
                        now += n;
                        now -= 1;
                        now = (now + n) % n;
                        ret += 1;
                        if now == l {
                            break;
                        }
                    }
                    if now == t {
                        Some(ret)
                    } else {
                        None
                    }
                };
                r = t;
                if let (Some(val0), Some(val1)) = (v0, v1) {
                    ans += val0.min(val1);
                } else if let Some(val) = v0 {
                    ans += val;
                } else if let Some(val) = v1 {
                    ans += val;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
