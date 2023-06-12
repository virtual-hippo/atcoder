use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut s = vec![a[0]];
    let mut current = 0;
    for i in 1..n {
        if a[i] + 1 == a[i-1] || a[i] == a[i-1] {
            s[current] += a[i];
        } else {
            current += 1;
            s.push(a[i]);
        }
    }
    if a[n-1] == 0 && a[0] == m - 1 && s.len() > 1 {
        s[0] += s[s.len()-1];
        s.pop();
    }
    let max = s.iter().fold(0, |max, x| std::cmp::max(max, *x));
    let s2 = a.iter().sum::<u64>();
    println!("{}", s2-max);
}

